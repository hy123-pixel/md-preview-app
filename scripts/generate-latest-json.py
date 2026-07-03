#!/usr/bin/env python3
"""
生成 latest.json 并上传到 GitHub Release。

用法:
    python3 scripts/generate-latest-json.py <version>

环境变量:
    RELEASE_TOKEN 或 GITHUB_TOKEN
"""

import json
import sys
import urllib.request
import urllib.error
import os
import time

try:
    sys.stdout.reconfigure(encoding="utf-8")
    sys.stderr.reconfigure(encoding="utf-8")
except (AttributeError, OSError):
    pass

REPO = "hy123-pixel/md-preview-app"
MAX_RETRIES = 6
RETRY_DELAY = 10


def api_request(url, token, method="GET", data=None, retries=3, backoff=5, ignore_404=False):
    headers = {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    if data is not None:
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, headers=headers, method=method)
    if data is not None:
        req.data = data if isinstance(data, bytes) else data.encode()

    last_err = None
    for attempt in range(1, retries + 1):
        try:
            with urllib.request.urlopen(req) as resp:
                return json.loads(resp.read().decode())
        except urllib.error.HTTPError as e:
            if ignore_404 and e.code == 404:
                return None
            if e.code in (401, 403, 404, 422):
                raise
            last_err = e
        except Exception as e:
            last_err = e
        if attempt < retries:
            print(f"  API retry in {backoff}s ({attempt}/{retries})...")
            time.sleep(backoff)
    raise last_err


def download_url(url):
    with urllib.request.urlopen(url) as resp:
        return resp.read()


def find_asset(assets, suffixes):
    for asset in assets:
        for suffix in suffixes:
            if asset["name"].endswith(suffix):
                return asset
    return None


def fetch_release(tag, token):
    url = f"https://api.github.com/repos/{REPO}/releases/tags/{tag}"
    for attempt in range(1, MAX_RETRIES + 1):
        release = api_request(url, token)
        assets = release.get("assets", [])
        if find_asset(assets, [".app.tar.gz.sig", ".msi.sig"]):
            return release
        print(f"[generate-latest-json] Assets not synced ({attempt}/{MAX_RETRIES}), waiting {RETRY_DELAY}s...")
        time.sleep(RETRY_DELAY)
    return release


def main():
    if len(sys.argv) < 2:
        print("Usage: generate-latest-json.py <version>", file=sys.stderr)
        sys.exit(1)

    version = sys.argv[1]
    token = os.environ.get("RELEASE_TOKEN") or os.environ.get("GITHUB_TOKEN")
    if not token:
        print("ERROR: RELEASE_TOKEN or GITHUB_TOKEN not set", file=sys.stderr)
        sys.exit(1)

    tag = f"v{version}"
    print(f"[generate-latest-json] Fetching release {tag}...")
    release = fetch_release(tag, token)
    assets = release["assets"]

    platforms = {}

    macos_sig = find_asset(assets, [".app.tar.gz.sig"])
    macos_tar = find_asset(assets, [".app.tar.gz"])
    if macos_sig and macos_tar:
        sig = download_url(macos_sig["browser_download_url"]).decode("utf-8")
        platforms["darwin-aarch64"] = {"signature": sig, "url": macos_tar["browser_download_url"]}
        print(f"  + macOS: {macos_tar['name']}")

    win_sig = find_asset(assets, [".msi.sig"])
    win_msi = find_asset(assets, [".msi"])
    if win_sig and win_msi:
        sig = download_url(win_sig["browser_download_url"]).decode("utf-8")
        platforms["windows-x86_64"] = {"signature": sig, "url": win_msi["browser_download_url"]}
        print(f"  + Windows: {win_msi['name']}")

    if not platforms:
        print("WARNING: No updater assets found", file=sys.stderr)
        sys.exit(0)

    latest_json = {
        "version": tag,
        "notes": release.get("body") or "",
        "pub_date": release.get("published_at") or "",
        "platforms": platforms,
    }

    with open("latest.json", "w") as f:
        json.dump(latest_json, f, indent=2)

    print(f"[generate-latest-json] Generated latest.json ({len(platforms)} platforms)")

    existing = find_asset(assets, ["latest.json"])
    if existing:
        api_request(f"https://api.github.com/repos/{REPO}/releases/assets/{existing['id']}", token, method="DELETE", ignore_404=True)

    upload_url = release["upload_url"].replace("{?name,label}", "") + "?name=latest.json"
    with open("latest.json", "rb") as f:
        data = f.read()

    req = urllib.request.Request(
        upload_url,
        data=data,
        headers={
            "Authorization": f"Bearer {token}",
            "Accept": "application/vnd.github+json",
            "Content-Type": "application/json",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        method="POST",
    )
    with urllib.request.urlopen(req) as resp:
        print(f"[generate-latest-json] Uploaded: HTTP {resp.status}")


if __name__ == "__main__":
    main()
