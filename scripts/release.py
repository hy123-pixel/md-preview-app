#!/usr/bin/env python3
"""
MD Preview 一键发布脚本（含 GitHub Actions 轮询）。

用法：
    python3 scripts/release.py [patch|minor|major|<version>]

流程：
1. 检查当前在 main 分支且工作区干净
2. 同步修改版本号文件
3. 创建 Tag 并推送，触发 GitHub Actions 构建
4. 轮询 Actions 状态直到完成

环境变量：
    GH_TOKEN 或 RELEASE_TOKEN
"""

import argparse
import json
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request

SOURCE_REPO = "hy123-pixel/md-preview-app"
RELEASE_REPO = "hy123-pixel/md-preview-app"
POLL_INTERVAL = 15
POLL_MAX_WAIT = 30 * 60


def get_token():
    # 优先从 md-preview-app 自身的 .env 读取
    env_file = os.path.join(os.path.dirname(__file__), "..", ".env")
    if os.path.isfile(env_file):
        with open(env_file, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if line.startswith("RELEASE_TOKEN="):
                    os.environ["RELEASE_TOKEN"] = line[len("RELEASE_TOKEN="):]
                    break

    # 回退到 finallshell-app 的 .env（复用同目录下的 token）
    if not os.environ.get("RELEASE_TOKEN") and not os.environ.get("GH_TOKEN") and not os.environ.get("GITHUB_TOKEN"):
        finallshell_env = os.path.join(os.path.dirname(__file__), "..", "..", "finalshell-app", ".env")
        if os.path.isfile(finallshell_env):
            with open(finallshell_env, "r", encoding="utf-8") as f:
                for line in f:
                    line = line.strip()
                    if line.startswith("RELEASE_TOKEN="):
                        os.environ["RELEASE_TOKEN"] = line[len("RELEASE_TOKEN="):]
                        break

    for key in ("GH_TOKEN", "RELEASE_TOKEN", "GITHUB_TOKEN"):
        token = os.environ.get(key)
        if token:
            return token
    print("错误：请设置 GH_TOKEN 或 RELEASE_TOKEN 环境变量", file=sys.stderr)
    sys.exit(1)


def github_api(method, url, token, data=None, timeout=60, retries=3, backoff=5):
    headers = {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "md-preview-release-script",
    }
    body = None
    if data is not None:
        body = json.dumps(data, ensure_ascii=False).encode()
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, data=body, headers=headers, method=method)

    last_err = None
    for attempt in range(1, retries + 1):
        try:
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                return resp.status, json.load(resp) if resp.status != 204 else None
        except urllib.error.HTTPError as e:
            err = e.read().decode()
            if e.code in (401, 403, 404, 422):
                raise
            print(f"  GitHub API 请求失败（HTTP {e.code}）{err[:200]}", file=sys.stderr)
            last_err = e
        except Exception as e:
            print(f"  GitHub API 请求失败（{type(e).__name__}: {e}）", file=sys.stderr)
            last_err = e

        if attempt < retries:
            print(f"  等待 {backoff} 秒后重试...（{attempt}/{retries}）")
            time.sleep(backoff)
        else:
            print(f"  重试次数用尽，放弃。", file=sys.stderr)
            raise last_err


def current_version():
    with open("package.json", "r", encoding="utf-8") as f:
        return json.load(f)["version"]


def bump_version(current, part):
    match = re.match(r"(\d+)\.(\d+)\.(\d+)", current)
    if not match:
        raise ValueError(f"无法解析版本号：{current}")
    major, minor, patch = (int(x) for x in match.groups())
    if part == "major":
        major += 1; minor = 0; patch = 0
    elif part == "minor":
        minor += 1; patch = 0
    elif part == "patch":
        patch += 1
    else:
        raise ValueError(f"不支持的版本递增：{part}")
    return f"{major}.{minor}.{patch}"


def set_version(version):
    with open("package.json", "r", encoding="utf-8") as f:
        pkg = json.load(f)
    pkg["version"] = version
    with open("package.json", "w", encoding="utf-8") as f:
        json.dump(pkg, f, indent=2, ensure_ascii=False)
        f.write("\n")

    with open("src-tauri/Cargo.toml", "r", encoding="utf-8") as f:
        cargo = f.read()
    cargo = re.sub(r'^version = "[^"]+"', f'version = "{version}"', cargo, count=1, flags=re.MULTILINE)
    with open("src-tauri/Cargo.toml", "w", encoding="utf-8") as f:
        f.write(cargo)

    with open("src-tauri/tauri.conf.json", "r", encoding="utf-8") as f:
        conf = json.load(f)
    conf["version"] = version
    with open("src-tauri/tauri.conf.json", "w", encoding="utf-8") as f:
        json.dump(conf, f, indent=2, ensure_ascii=False)
        f.write("\n")


def run(cmd, check=True):
    print(f"$ {' '.join(cmd)}")
    result = subprocess.run(cmd)
    if check and result.returncode != 0:
        sys.exit(result.returncode)
    return result


def poll_workflow_status(token, head_sha, version):
    url = f"https://api.github.com/repos/{SOURCE_REPO}/actions/runs?per_page=20"
    elapsed = 0
    target_names = {"Build macOS", "Build Windows"}
    found_runs = {}

    print(f"\n⏳ 轮询 GitHub Actions 状态（commit: {head_sha[:8]}）...")
    print(f"   最大等待 {POLL_MAX_WAIT // 60} 分钟\n")

    while elapsed < POLL_MAX_WAIT:
        _, data = github_api("GET", url, token)
        runs = data.get("workflow_runs", [])

        for run in runs:
            name = run.get("name", "")
            if name not in target_names:
                continue
            if run.get("head_sha") != head_sha:
                continue
            found_runs[name] = run

        if not found_runs:
            print(f"  [{elapsed // 60:02d}m{elapsed % 60:02d}s] 等待 Actions 启动...")
            time.sleep(POLL_INTERVAL)
            elapsed += POLL_INTERVAL
            continue

        statuses = []
        for name in sorted(target_names):
            run = found_runs.get(name)
            if run is None:
                statuses.append(f"{name}: 等待启动")
            else:
                status = run.get("status", "unknown")
                conclusion = run.get("conclusion", "-")
                if status == "completed":
                    icon = "✅" if conclusion == "success" else "❌"
                    statuses.append(f"{name}: {icon} {conclusion}")
                else:
                    statuses.append(f"{name}: ⏳ {status}")
        print(f"  [{elapsed // 60:02d}m{elapsed % 60:02d}s] " + "  |  ".join(statuses))

        all_completed = all(r.get("status") == "completed" for r in found_runs.values())
        if all_completed and len(found_runs) == len(target_names):
            break

        time.sleep(POLL_INTERVAL)
        elapsed += POLL_INTERVAL

    if elapsed >= POLL_MAX_WAIT:
        print(f"\n❌ 超时", file=sys.stderr)
        sys.exit(1)

    failures = [(n, r.get("conclusion"), r.get("html_url")) for n, r in found_runs.items() if r.get("conclusion") != "success"]
    if failures:
        print(f"\n❌ 发布失败", file=sys.stderr)
        for name, conclusion, url in failures:
            print(f"   - {name}: {conclusion}\n     {url}", file=sys.stderr)
        sys.exit(1)

    print(f"\n✅ 全部构建成功！")
    print(f"\n🎉 发布完成： https://github.com/{RELEASE_REPO}/releases/tag/v{version}")


def main():
    parser = argparse.ArgumentParser(description="MD Preview 一键发布")
    parser.add_argument("version", nargs="?", help="新版本号或 patch/minor/major")
    args = parser.parse_args()

    token = get_token()
    current = current_version()

    version_input = args.version or input(f"当前版本 {current}，输入新版本（或 patch/minor/major）：").strip()

    if version_input in ("patch", "minor", "major"):
        new_version = bump_version(current, version_input)
    else:
        if not re.match(r"\d+\.\d+\.\d+$", version_input):
            print("错误：版本号必须是 x.y.z 格式", file=sys.stderr)
            sys.exit(1)
        new_version = version_input

    print(f"\n准备发布：{current} -> {new_version}\n")

    branch = subprocess.check_output(["git", "branch", "--show-current"]).decode().strip()
    if branch != "main":
        print(f"错误：当前分支 {branch}，请切换到 main", file=sys.stderr)
        sys.exit(1)

    set_version(new_version)

    run(["git", "add", "package.json", "src-tauri/Cargo.toml", "src-tauri/tauri.conf.json"])
    run(["git", "commit", "-m", f"release: bump version to v{new_version}"])
    run(["git", "push", "origin", "main"])

    subprocess.run(["git", "tag", "-d", f"v{new_version}"], check=False)
    run(["git", "tag", f"v{new_version}"])
    run(["git", "push", "origin", f"v{new_version}"])

    head_sha = subprocess.check_output(["git", "rev-parse", "HEAD"]).decode().strip()
    print(f"\n🚀 已推送 Tag v{new_version}（{head_sha[:8]}），触发构建...")
    poll_workflow_status(token, head_sha, new_version)


if __name__ == "__main__":
    main()
