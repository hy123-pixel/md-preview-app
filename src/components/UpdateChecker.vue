<script setup lang="ts">
import { ref, onMounted } from "vue";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { ask } from "@tauri-apps/plugin-dialog";

const updateAvailable = ref(false);
const updateVersion = ref("");
const updateNotes = ref("");
const isUpdating = ref(false);
const updateProgress = ref(0);

let updateHandle: any = null;

onMounted(async () => {
  // 延迟 3 秒后检查更新，避免启动时阻塞
  setTimeout(() => {
    checkForUpdate();
  }, 3000);
});

async function checkForUpdate() {
  try {
    updateHandle = await check();
    if (updateHandle) {
      updateAvailable.value = true;
      updateVersion.value = updateHandle.version;
      updateNotes.value = updateHandle.body || "";
      
      const shouldUpdate = await ask(
        `发现新版本 ${updateHandle.version}，是否立即更新？\n\n${updateNotes.value || ""}`,
        { title: "更新可用", kind: "info" }
      );
      
      if (shouldUpdate) {
        await doUpdate();
      }
    }
  } catch (e) {
    console.log("更新检查失败（非 Tauri 环境或网络问题）:", e);
  }
}

async function doUpdate() {
  if (!updateHandle) return;
  
  isUpdating.value = true;
  try {
    let downloaded = 0;
    let contentLength = 0;
    
    await updateHandle.downloadAndInstall((event: any) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength || 0;
          console.log(`开始下载更新，大小: ${contentLength} bytes`);
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          if (contentLength > 0) {
            updateProgress.value = Math.round((downloaded / contentLength) * 100);
          }
          break;
        case "Finished":
          console.log("下载完成");
          break;
      }
    });
    
    console.log("更新安装完成，准备重启...");
    await relaunch();
  } catch (e) {
    console.error("更新失败:", e);
    isUpdating.value = false;
    
    await ask(
      `更新失败: ${e}`,
      { title: "更新错误", kind: "error" }
    );
  }
}

async function manualCheck() {
  updateAvailable.value = false;
  updateVersion.value = "";
  updateNotes.value = "";
  await checkForUpdate();
  
  if (!updateAvailable.value) {
    await ask("当前已是最新版本", { title: "检查更新", kind: "info" });
  }
}
</script>

<template>
  <div class="update-checker">
    <!-- 更新中弹窗 -->
    <div v-if="isUpdating" class="update-overlay">
      <div class="update-modal">
        <div class="update-title">正在下载更新...</div>
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: updateProgress + '%' }"></div>
        </div>
        <div class="progress-text">{{ updateProgress }}%</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.update-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}

.update-modal {
  background: #fff;
  border-radius: 8px;
  padding: 24px 32px;
  min-width: 300px;
  text-align: center;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

:global(.dark) .update-modal {
  background: #2d2d2d;
  color: #ddd;
}

.update-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #333;
}

:global(.dark) .update-title {
  color: #ddd;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

:global(.dark) .progress-bar {
  background: #444;
}

.progress-fill {
  height: 100%;
  background: #007acc;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 13px;
  color: #666;
}

:global(.dark) .progress-text {
  color: #aaa;
}
</style>
