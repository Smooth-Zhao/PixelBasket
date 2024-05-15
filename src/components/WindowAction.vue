<script setup lang="ts">
import {NIcon} from "naive-ui";
import {RemoveOutline,CloseOutline} from "@vicons/ionicons5";
import {FullScreenMaximize20Filled} from "@vicons/fluent"
import {exit} from "@tauri-apps/api/process";
import {appWindow} from "@tauri-apps/api/window";
const handleCloseWindow = () => {
  if (appWindow.label === "main"){
    exit(1)
  }else{
    appWindow.close()
  }
}
const handleToggleMaximize = () => {
  appWindow.toggleMaximize()
}
const handleMinimize = () => {
  appWindow.minimize()
}
</script>

<template>
  <div class="window-action">
    <div class="action-item" @click="handleMinimize">
      <n-icon :size="20">
        <RemoveOutline></RemoveOutline>
      </n-icon>
    </div>
    <div class="action-item" @click="handleToggleMaximize">
      <n-icon :size="16">
        <FullScreenMaximize20Filled></FullScreenMaximize20Filled>
      </n-icon>
    </div>
    <div class="action-item" @click="handleCloseWindow">
      <n-icon :size="20">
        <CloseOutline></CloseOutline>
      </n-icon>
    </div>
  </div>
</template>

<style scoped lang="scss">
.window-action {
  display: flex;
  position: fixed;
  top: 2px;
  right: 2px;
  .action-item {
    width: 30px;
    height: 30px;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    transition: background-color .2s ease;
    &:hover{
      background-color: var(--color-dark-3);
    }
  }
}
</style>
