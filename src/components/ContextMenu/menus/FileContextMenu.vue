<script setup lang="ts">
import ContextMenuItem from "../components/ContextMenuItem.vue";
import ContextMenuGroup from "../components/ContextMenuGroup.vue";
import ContextMenu from "../index.vue"
import {IContextMenuProps} from "../createContextMenu.ts";
import useSelection from "../../../hooks/useSelection.ts";
import {openFile} from "../../../utils";
defineProps<IContextMenuProps<any>>()
const selection = useSelection()
function handleOpen() {
  const file = Array.from(selection.items.value)[0];
  openFile(file)
}
</script>

<template>
  <context-menu v-bind="$props">
    <context-menu-group>
      <context-menu-item shortcut="Alt + F5" @click="handleOpen">
        打开
      </context-menu-item>
    </context-menu-group>
  </context-menu>
</template>

<style scoped lang="scss"></style>
