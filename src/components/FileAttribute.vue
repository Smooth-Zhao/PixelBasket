<script setup lang="ts">
import useSelection from "../hooks/useSelection.ts";
import {NScrollbar,NTooltip,useMessage,NInput,NSpace,NSelect,NRate} from "naive-ui";
import FilePreview from "./ContentBrowser/components/FilePreview.vue";
import {computed} from "vue";

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
const previewSrc = computed(()=>{
  return Array.from(items.value).at(-1)
})</script>

<template>
  <n-scrollbar>
    <div class="file-attribute">
      <div class="preview-image">
        <file-preview v-if="previewSrc" :src="previewSrc"/>
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
      <n-space class="file-attribute-edit" vertical>
        <div class="file-name-input">
          <label>文件名：</label>
          <n-input />
        </div>
        <div class="file-name-input">
          <label>标签：</label>
          <n-select
            filterable
            multiple
            tag
          />
        </div>
        <div class="file-name-input">
          <label>备注：</label>
          <n-input/>
        </div>
      </n-space>
      <div class="details">
        <div class="detail-item">
          <span>评分</span>
          <span> <n-rate :size="12" /></span>
        </div>
        <div class="detail-item">
          <span>文件大小</span>
          <span>10kb</span>
        </div>
        <div class="detail-item">
          <span>尺寸</span>
          <span>1204 × 1358</span>
        </div>
        <div class="detail-item">
          <span>格式</span>
          <span>JPG</span>
        </div>
        <div class="detail-item">
          <span>创建日期</span>
          <span>2024-4-10 16:30:11</span>
        </div>
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
    gap: 2px;
    span{
      display: block;
      flex: 1 1 auto;
      height: 10px;
      background-color: var(--item-color);
      cursor: pointer;
    }
  }
  .file-attribute-edit{
    margin-top: 16px;
  }
  .details{
    margin-top: 36px;
    padding: 8px;
    .detail-item{
      display: flex;
      font-size: 12px;
      span{
        width: 0;
      }
      span:nth-child(1){
        flex: 2 2 auto;
      }
      span:nth-child(2){
        flex: 3 3 auto;
        color:var(--color-gray-2);
      }
    }
  }
}
</style>
