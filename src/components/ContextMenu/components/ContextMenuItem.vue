<script setup lang="ts">
import {CaretRight20Filled} from '@vicons/fluent'
import {NIcon} from 'naive-ui'
</script>

<template>
  <div class="context-menu-item">
    <slot/>
    <span class="shortcut">
      Alt + F5
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
