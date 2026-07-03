import { ref } from "vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import { ask } from "@tauri-apps/plugin-dialog";
import type { FileNode } from "../stores/editor";

export function useFile() {
  const isLoading = ref(false);

  /** 打开文件 */
  async function openFile(): Promise<{ path: string; name: string; content: string } | null> {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [
          { name: "Markdown", extensions: ["md", "markdown", "txt"] },
          { name: "All Files", extensions: ["*"] },
        ],
      });
      if (!selected) return null;

      isLoading.value = true;
      const path = selected as string;
      const { invoke } = await import("@tauri-apps/api/core");
      const content = await invoke<string>("read_file", { path });
      const name = path.split("/").pop() || path.split("\\").pop() || "未命名.md";
      isLoading.value = false;
      return { path, name, content };
    } catch (e: any) {
      isLoading.value = false;
      console.error("Open file error:", e);
      const msg = e?.toString() || String(e);
      if (msg.includes("permission") || msg.includes("denied") || msg.includes("not permitted")) {
        await ask(
          "无法访问该文件，请前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予 MD Preview 访问权限。",
          { title: "权限不足", kind: "warning" }
        );
      }
      return null;
    }
  }

  /** 保存文件 */
  async function saveFile(content: string, currentPath?: string | null): Promise<{ path: string; name: string } | null> {
    try {
      let path: string | null = currentPath || null;
      if (!path) {
        const selected = await save({
          filters: [{ name: "Markdown", extensions: ["md"] }],
          defaultPath: "未命名.md",
        });
        if (!selected) return null;
        path = selected;
      }

      isLoading.value = true;
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("write_file", { path, content });
      const name = path.split("/").pop() || path.split("\\").pop() || "未命名.md";
      isLoading.value = false;
      return { path, name };
    } catch (e: any) {
      isLoading.value = false;
      console.error("Save file error:", e);
      const msg = e?.toString() || String(e);
      if (msg.includes("permission") || msg.includes("denied") || msg.includes("not permitted")) {
        await ask(
          "无法保存文件，请前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予 MD Preview 访问权限。",
          { title: "权限不足", kind: "warning" }
        );
      } else {
        await ask(`保存失败: ${msg}`, { title: "保存错误", kind: "error" });
      }
      return null;
    }
  }

  /** 另存为 */
  async function saveAsFile(content: string): Promise<{ path: string; name: string } | null> {
    try {
      const selected = await save({
        filters: [{ name: "Markdown", extensions: ["md"] }],
        defaultPath: "未命名.md",
      });
      if (!selected) return null;

      isLoading.value = true;
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("write_file", { path: selected, content });
      const name = selected.split("/").pop() || selected.split("\\").pop() || "未命名.md";
      isLoading.value = false;
      return { path: selected, name };
    } catch (e: any) {
      isLoading.value = false;
      console.error("Save as error:", e);
      const msg = e?.toString() || String(e);
      if (msg.includes("permission") || msg.includes("denied") || msg.includes("not permitted")) {
        await ask(
          "无法保存文件，请前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予 MD Preview 访问权限。",
          { title: "权限不足", kind: "warning" }
        );
      } else {
        await ask(`保存失败: ${msg}`, { title: "保存错误", kind: "error" });
      }
      return null;
    }
  }

  /** 读取文件夹 */
  async function readFolder(folderPath: string): Promise<FileNode[]> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const entries = await invoke<Array<{ name: string; path: string; is_directory: boolean }>>("read_dir", { path: folderPath });
      const result: FileNode[] = [];

      for (const entry of entries) {
        const node: FileNode = {
          name: entry.name,
          path: entry.path,
          isDirectory: entry.is_directory,
        };

        if (entry.is_directory) {
          node.children = await readFolder(node.path);
        }

        result.push(node);
      }

      return result.sort((a, b) => {
        if (a.isDirectory !== b.isDirectory) {
          return a.isDirectory ? -1 : 1;
        }
        return a.name.localeCompare(b.name);
      });
    } catch (e) {
      console.error("Read folder error:", e);
      return [];
    }
  }

  /** 打开文件夹 */
  async function openFolder(): Promise<{ path: string; tree: FileNode[] } | null> {
    try {
      const selected = await open({
        multiple: false,
        directory: true,
      });
      if (!selected) return null;

      const path = selected as string;
      const tree = await readFolder(path);
      return { path, tree };
    } catch (e) {
      console.error("Open folder error:", e);
      return null;
    }
  }

  return {
    isLoading,
    openFile,
    saveFile,
    saveAsFile,
    openFolder,
    readFolder,
  };
}
