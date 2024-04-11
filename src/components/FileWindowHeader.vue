<script setup lang="ts">

import WindowAction from "./WindowAction.vue";
</script>

<template>
<div class="file-window-header" data-tauri-drag-region>
  1235.JPG（1920 x 1080）
  <window-action/>
</div>
</template>

<style scoped lang="scss">
.file-window-header{
  flex: 0 0 34px;
  padding: 0 8px;
  font-size: 14px;
  line-height: 34px;
  border-bottom: solid 1px var(--color-dark-3);
  display: flex;
  align-items: center;
}
</style>
