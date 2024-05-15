<script setup lang="ts">
import {ref} from "vue";
import {onClickOutside} from '@vueuse/core'
import {contextMenuMap} from "./createContextMenu.ts";

const props = defineProps<{
  position: {
    x: number,
    y: number
  },
  show: boolean,
  payload: any,
  name?: string
}>()

const target = ref(null)
onClickOutside(target, () => {
  hide()
})
const hide = () => {
  if (props.name) {
    const componentProps = contextMenuMap.get(props.name)
    if (componentProps) componentProps.props.value.show = false
  }
}
</script>

<template>
  <div class="context-menu" v-if="show" ref="target" :style="{
    left:position.x+'px',
    top:position.y+'px'
  }"
       @click="hide"
  >
    <slot/>
  </div>
</template>

<style scoped lang="scss"></style>
