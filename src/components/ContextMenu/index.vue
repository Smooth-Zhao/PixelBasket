<script setup lang="ts">
import {ref, watch, computed, provide} from "vue";
import {onClickOutside} from '@vueuse/core'
import {activeContextMenuData} from "./createContextMenu.ts";
import SubContextMenu from "./components/SubContextMenu.vue";
import {contextMenuHandlerKey} from "../../utils/injectionKeys.ts";

const visible = ref(false)
const target = ref(null)
onClickOutside(target, () => visible.value = false)

const contextMenuPosition = computed(() => {
  if (!activeContextMenuData.value) return
  return {
    left: `${activeContextMenuData.value.position.x + 5}px`,
    top: `${activeContextMenuData.value.position.y + 5}px`,
    display: "block"
  }
})

const handlerExecute = ()=>{
  visible.value = false
}

provide(contextMenuHandlerKey,handlerExecute)

watch(activeContextMenuData, () => {
  visible.value = true
})

</script>

<template>
  <sub-context-menu
    ref="target"
    v-if="visible"
    :style="contextMenuPosition"
    :data="activeContextMenuData!.option.menu"
  />
</template>

<style scoped lang="scss"></style>
