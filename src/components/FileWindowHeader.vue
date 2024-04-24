<script setup lang="ts">

import WindowAction from "./WindowAction.vue";
import PBFile from "../entities/PBFile.ts";
defineProps<{
  file: PBFile
}>()
</script>

<template>
  <div v-if="file" class="file-window-header" data-tauri-drag-region>
    {{ file.fileName }}.{{ file.fileSuffix }}
    <window-action/>
  </div>
</template>

<style scoped lang="scss">
.file-window-header {
  flex: 0 0 34px;
  padding: 0 8px;
  font-size: 14px;
  line-height: 34px;
  border-bottom: solid 1px var(--color-dark-3);
  display: flex;
  align-items: center;
}
</style>
