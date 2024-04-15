<script setup lang="ts">
import Category from "./Category/index.vue";
import ContentBrowser from "./ContentBrowser/index.vue";
import ContentFooter from "./ContentFooter.vue";
import ContentHeader from "./ContentHeader.vue";
import useMainContextMenu from "../hooks/useMainContextMenu.ts";
import FileAttribute from "./FileAttribute.vue";
// import Dev from "./Dev.vue";
const { trigger } = useMainContextMenu()
</script>

<template>
  <div class="layout">
    <div class="layout-left">
      <category/>
    </div>
    <div class="layout-center" @contextmenu="trigger">
      <content-header/>
      <content-browser/>
      <content-footer/>
    </div>
    <div class="layout-right">
      <file-attribute/>
    </div>
  </div>
<!--  <dev/>-->
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
    display: flex;
    flex-direction: column;
  }

  .layout-right {
    width: 0;
    flex: 0 0 240px;
    border-left: solid 1px var(--color-dark-3);
    background-color: var(--color-dark-1);
  }
}

</style>
