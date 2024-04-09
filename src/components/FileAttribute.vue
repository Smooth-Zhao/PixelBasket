<script setup lang="ts">
import useSelection from "../hooks/useSelection.ts";
import {NScrollbar,NTooltip,useMessage} from "naive-ui";

const {items} = useSelection()
const message = useMessage()
const handleColorItemClick = (color:string) => {
  //复制到剪贴板
  navigator.clipboard.writeText(color)
    .then(() => {
      message.success('复制成功')
    })
    .catch(() => {
      message.error('复制失败')
    })
}
</script>

<template>
  <n-scrollbar>
    <div class="file-attribute">
      <div class="preview-image">
        <img style="width: 100%" :src="Array.from(items).at(-1)">
      </div>
      <div class="main-colors">
        <n-tooltip trigger="hover">
          <template #trigger>
            <span style="--item-color:#c44343" @click="handleColorItemClick('#c44343')"></span>
          </template>
          #c44343
        </n-tooltip>

        <span style="--item-color:#0effc3"></span>
        <span style="--item-color:#9916bb"></span>
        <span style="--item-color:#52a275"></span>
        <span style="--item-color:#472abd"></span>
        <span style="--item-color:#4da1dc"></span>
        <span style="--item-color:#4da1dc"></span>
      </div>
    </div>
  </n-scrollbar>
</template>

<style scoped lang="scss">
.file-attribute{
  height: 100%;
  padding: 56px 16px 16px;
  .preview-image{
    width: 100%;
    height: 160px;
    margin-bottom: 8px;
    img{
      width: 100%;
      height: 100%;
      display: block;
      object-fit: contain;
    }
  }
  .main-colors{
    display: flex;
    gap: 4px;
    span{
      display: block;
      flex: 1 1 auto;
      height: 14px;
      border-radius: 2px;
      background-color: var(--item-color);
      cursor: pointer;
    }
  }
}
</style>
