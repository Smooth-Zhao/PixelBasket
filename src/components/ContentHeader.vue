<script setup lang="ts">
import { NInput, NIcon} from "naive-ui"
import {Filter20Filled} from "@vicons/fluent"
import Filter from "./ContentBrowser/components/FileFilter.vue"
import {ref} from "vue";
import useFolder from "../hooks/useFolder.ts";

const expand = ref(false)
const handleFilterExpand = () => {
  expand.value = !expand.value
}
const folder = useFolder()
</script>

<template>
  <div class="content-header" :class="{expand}">
    <div class="tools-bar">
      <div class="search">
        <n-input type="text" size="tiny" placeholder="关键字查询"/>
      </div>
      <div class="tauri-drag-region" data-tauri-drag-region></div>
      <div class="tools-container">
        <div class="tool-button filter" @click="handleFilterExpand">
          <n-icon :size="18">
            <Filter20Filled/>
          </n-icon>
        </div>
      </div>
    </div>
    <Filter/>
  </div>
</template>

<style scoped lang="scss">
.content-header {
  position: relative;
  flex: 0 0 40px;
  transition: flex .2s ease;
  overflow: hidden;
  padding: 0 8px;
  will-change: auto;

  .tauri-drag-region{
    height: 40px;
    flex: 1 1 auto;
  }

  .tools-bar {
    display: flex;
  }

  .search {
    flex: 0 0 200px;
    height: 40px;
    display: flex;
    align-items: center;
  }

  .tools-container {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 8px;

    .tool-button {
      width: 20px;
      height: 20px;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      background-color: var(--color-dark-1);
      transition: background-color .2s ease;
      border-radius: 4px;

      &:hover {
        background-color: var(--color-dark-3);
      }
    }
  }

  &.expand {
    flex: 0 0 68px;
    .tool-button.filter{
      background-color: var(--color-dark-3);
    }
  }
}
</style>
