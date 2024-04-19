<script setup lang="ts">
import useSelection from "../hooks/useSelection.ts";
import {NScrollbar, NTooltip, useMessage, NInput, NSpace, NSelect, NRate, NEllipsis} from "naive-ui";
import FilePreview from "./ContentBrowser/components/FilePreview.vue";
import {computed} from "vue";
import PBFile from "../entities/PBFile.ts";
import {bytesToSize} from "../utils";

const {items} = useSelection()
const message = useMessage()
const handleColorItemClick = (color: string) => {
  //复制到剪贴板
  navigator.clipboard.writeText(color)
    .then(() => {
      message.success('复制成功')
    })
    .catch(() => {
      message.error('复制失败')
    })
}
const file = computed<PBFile>(() => {
  return Array.from(items.value).at(-1)
})
const colors = computed(() => {
  return file.value?.colors.split(",").slice(1, 7)
})
</script>

<template>
  <n-scrollbar v-if="file">
    <div class="file-attribute">
      <div class="preview-image">
        <file-preview v-if="file" :file="file"/>
      </div>
      <div class="main-colors">
        <n-tooltip trigger="hover" v-for="item in colors">
          <template #trigger>
            <span :style="{'--item-color':item}" @click="handleColorItemClick(item)"></span>
          </template>
          {{ item }}
        </n-tooltip>
      </div>
      <n-space class="file-attribute-edit" vertical>
        <div class="file-name-input">
          <label>文件名：</label>
          <n-input v-model:value="file.fileName" placeholder=""/>
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
          <span> <n-rate v-model:value="file.score" :size="12"/></span>
        </div>
        <div class="detail-item">
          <span>文件大小</span>
          <span>{{ bytesToSize(file.fileSize) }}</span>
        </div>
        <div class="detail-item">
          <span>尺寸</span>
          <span>{{ file.imageWidth }} * {{ file.imageHeight }}</span>
        </div>
        <div class="detail-item">
          <span>格式</span>
          <span>{{ file.fileSuffix }}</span>
        </div>
        <div class="detail-item">
          <span>创建日期</span>
          <span>{{ file.created }}</span>
        </div>
        <div class="detail-item">
          <span>文件路径</span>
          <span>
            <n-ellipsis :line-clamp="1" style="max-width: 100%">
              {{ file.fullPath }}
            </n-ellipsis>
          </span>
        </div>
      </div>
    </div>
  </n-scrollbar>
</template>

<style scoped lang="scss">
.file-attribute {
  height: 100%;
  padding: 56px 16px 16px;

  .preview-image {
    width: 100%;
    height: 160px;
    margin-bottom: 8px;

    img {
      width: 100%;
      height: 100%;
      display: block;
      object-fit: contain;
    }
  }

  .main-colors {
    display: flex;
    gap: 2px;

    span {
      display: block;
      flex: 1 1 auto;
      height: 10px;
      background-color: var(--item-color);
      cursor: pointer;
    }
  }

  .file-attribute-edit {
    margin-top: 16px;
  }

  .details {
    margin-top: 36px;
    padding: 4px;

    .detail-item {
      display: flex;
      font-size: 14px;

      span {
        width: 0;
      }

      span:nth-child(1) {
        flex: 2 2 auto;
      }

      ::v-deep(span:nth-child(2)) {
        flex: 3 3 auto;
        width: 0;
        color: var(--color-gray-2);
      }
    }
  }
}
</style>
