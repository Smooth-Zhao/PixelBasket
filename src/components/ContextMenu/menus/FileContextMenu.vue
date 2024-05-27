<script setup lang="ts">
import ContextMenuItem from "../components/ContextMenuItem.vue";
import ContextMenuGroup from "../components/ContextMenuGroup.vue";
import ContextMenu from "../index.vue"
import {IContextMenuProps} from "../createContextMenu.ts";
import useSelection from "../../../hooks/useSelection.ts";
import {openFile, openFileWithLocalProgram} from "../../../utils";
defineProps<IContextMenuProps<any>>()
const selection = useSelection()
function handleOpen() {
  const file = Array.from(selection.items.value)[0];
  openFile(file)
}
function handleOpenWithLocalProgram() {
  const file = Array.from(selection.items.value)[0];
  openFileWithLocalProgram(file.fullPath)
}
function handleOpenPath() {
  const file = Array.from(selection.items.value)[0];
  openFileWithLocalProgram(file.filePath)
}
</script>

<template>
  <context-menu v-bind="$props">
    <context-menu-group>
      <context-menu-item @click="handleOpen">
        打开
      </context-menu-item>
      <context-menu-item @click="handleOpenWithLocalProgram">
        使用默认程序打开
      </context-menu-item>
      <context-menu-item @click="handleOpenPath">
        打开所在文件夹
      </context-menu-item>
    </context-menu-group>
  </context-menu>
</template>

<style scoped lang="scss"></style>
