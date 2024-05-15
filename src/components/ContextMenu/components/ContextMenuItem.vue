<script setup lang="ts">
import {CaretRight20Filled} from '@vicons/fluent'
import {NIcon} from 'naive-ui'
defineProps<{
  shortcut?:string,
  prefix?:string,
  disabled?:boolean,
}>()
</script>

<template>
  <div class="context-menu-item" :class="{disabled}">
    <span class="prefix">
      <slot name="prefix" class="prefix">{{prefix}}</slot>
    </span>
    <slot/>
    <span class="shortcut" v-if="shortcut">
      {{ shortcut}}
    </span>
    <n-icon v-if="$slots['sub-menu']" size="12" class="icon-more">
      <CaretRight20Filled/>
    </n-icon>
    <slot name="sub-menu"/>
  </div>
</template>

<style scoped lang="scss">
.context-menu-item>::v-deep(.context-menu){
  display: none;
  left: 100%;
  top: 4px;
}
</style>
