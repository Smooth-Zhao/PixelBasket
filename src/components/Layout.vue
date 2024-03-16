<script setup lang="ts">

import Category from "./Category.vue";
</script>

<template>
  <div class="layout">
    <div class="layout-left">
      <category/>
    </div>
    <div class="layout-center"></div>
    <div class="layout-right"></div>
  </div>
</template>
<style scoped lang="scss">
.layout {
  display: flex;
  height: 100%;

  .layout-left {
    width: 0;
    flex: 0 0 240px;
    border-right: solid 1px var(--color-dark-3);
    background-color: var(--color-dark-2);
  }

  .layout-center {
    width: 0;
    flex: 1 1 auto;
    background-color: var(--color-dark-1);
  }

  .layout-right {
    width: 0;
    flex: 0 0 240px;
    border-left: solid 1px var(--color-dark-3);
    background-color: var(--color-dark-1);
  }
}

</style>