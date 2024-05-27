<script setup lang="ts">
import {NSpace, NIcon, NPopover, NCheckbox} from "naive-ui"
import {ChevronDown20Filled} from "@vicons/fluent"
import Color from "./filters/Color.vue"
import useContentBrowser from "../../../hooks/useContentBrowser.ts";
import {watch} from "vue";
import useFolder from "../../../hooks/useFolder.ts";

const filterItems = [{
  name: "颜色",
  component: Color
}, {
  name: "标签",
}, {
  name: "形状",
}, {
  name: "评分",
}, {
  name: "格式",
}, {
  name: "添加日期",
}, {
  name: "尺寸",
}, {
  name: "时长",
}, {
  name: "大小",
}, {
  name: "注释",
}]
const {isLoadChildren, load} = useContentBrowser()
const folder = useFolder()
watch(isLoadChildren, () => {
  load(folder.currentFolder.value)
})
</script>

<template>
  <div class="file-filter">
    <n-space :size="4">
      <n-popover v-for="item in filterItems" :overlap="false" placement="top-start" trigger="click">
        <template #trigger>
          <div class="filter-item">
            {{ item.name }}
            <n-icon :size="14">
              <ChevronDown20Filled/>
            </n-icon>
          </div>
        </template>
        <component v-if="item.component" :is="item.component"/>
      </n-popover>
      <div class="filter-item">
        <n-checkbox size="small" label="显示子文件" v-model:checked="isLoadChildren"/>
      </div>
    </n-space>
  </div>
</template>

<style scoped lang="scss">
.file-filter {
  .filter-item {
    cursor: pointer;
    border-radius: 4px;
    padding: 0 4px;
    display: flex;
    align-items: center;
    transition: background-color .3s ease-in-out;

    &:hover {
      background-color: var(--color-dark-2);
    }

    &.active {
      background-color: var(--color-gray-1);
    }
  }
}
</style>
