<script setup lang="ts">
import {contextMenuMap} from "./createContextMenu.ts";
</script>

<template>
  <slot/>
  <teleport to="body">
    <component
      :is="item[1].component"
      v-bind="item[1].props.value"
      v-for="item of contextMenuMap"
      :key="item[0]"
      :name="item[0]"
    />
  </teleport>
</template>

<style scoped lang="scss">

</style>
