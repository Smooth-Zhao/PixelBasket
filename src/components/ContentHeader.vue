<script setup lang="ts">
import {appWindow} from "@tauri-apps/api/window";

const handleDragWindow = (e: MouseEvent) => {
  appWindow.startDragging()
  e.stopPropagation()
  e.preventDefault()
}
</script>

<template>
  <div class="content-header" @mousemove="handleDragWindow">
  </div>
</template>

<style scoped lang="scss">
.content-header {
  height: 40px;
  transition: height .2s ease;
  overflow: hidden;
  border-bottom: solid 1px var(--color-dark-3);
  display: flex;
  align-items: center;
  padding: 0 16px;
}
</style>
