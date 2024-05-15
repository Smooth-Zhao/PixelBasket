<script setup lang="ts">
import {NColorPicker} from "naive-ui";
</script>

<template>
<div class="color-filter">
  <n-color-picker
    default-show
    :show="true"
    show-preview
    :to="false"
    :swatches="[
      '#FFFFFF',
      '#18A058',
      '#2080F0',
      '#F0A020',
      'rgba(208, 48, 80, 1)'
    ]"
  />
</div>
</template>

<style scoped lang="scss">
.color-filter{
  height: 290px;
  width:200px;
  ::v-deep(.n-color-picker-trigger){
    opacity: 0;
  }
  ::v-deep(.v-binder-follower-content){
    position: relative;
    transform:translate(0)!important;
  }
  ::v-deep(.n-color-picker-panel){
    box-shadow: none;
    padding: 0;
    width: 200px;
    .n-color-picker-control{
      width: 100%;
      padding: 0;
    }
  }
}
</style>
