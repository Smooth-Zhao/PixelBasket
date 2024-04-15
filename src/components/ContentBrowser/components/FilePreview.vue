<script setup lang="ts">
import {computed} from "vue";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {VideoViewer,ImageViewer} from "../../FileViewer";
import {getFileType} from "../../../utils";

const props = defineProps<{
  src: string,
  controls?:boolean
  showFileType?: boolean
}>()

const assetSrc = computed(() => convertFileSrc(props.src))

const suffixName = computed(() => {
  return props.src?.split('.').pop()?.toUpperCase();
})

const fileType = computed(() => {
  if (!suffixName.value) return;
  return getFileType(suffixName.value)
})

</script>

<template>
  <div class="file-preview">
    <img v-if="fileType === 'image'" :src="assetSrc" alt="">
    <video-viewer v-else-if="fileType === 'video'" :src="assetSrc"/>
    <span v-else>
      Unsupported File Type
    </span>

    <div v-if="fileType!== 'other' && showFileType" class="file-type-tag" :class="fileType">{{ suffixName }}</div>
  </div>
</template>

<style scoped lang="scss">
.file-preview {
  width: 100%;
  height: 100%;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;

  img{
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
  }


  .file-type-tag {
    position: absolute;
    right: 0;
    top: 0;
    padding: 0 10px;
    font-size: 12px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);

    &.image {
      background: rgb(231, 112, 83);
    }

    &.video {
      background: rgba(70, 172, 255);
    }
  }
}

</style>
