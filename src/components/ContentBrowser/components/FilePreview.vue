<script setup lang="ts">
import {computed} from "vue";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {getFileType} from "../../../utils";
import PBFile from "../../../entities/PBFile.ts";
import VideoPreview from "./VideoPreview.vue";

const props = defineProps<{
  file: PBFile,
  controls?:boolean
  showFileType?: boolean
}>()

const assetSrc = computed(() => {
  return convertFileSrc(props.file.fullPath)
})

const suffixName = computed(() => {
  return props.file.fileSuffix.toUpperCase();
})

const fileType = computed(() => {
  if (!suffixName.value) return "other";
  return getFileType(suffixName.value)
})

</script>

<template>
  <div class="file-preview">
    <img v-if=" ['image','encoded_image'].includes(fileType)" :src="props.file.thumbnail" alt="">
    <video-preview v-else-if="fileType === 'video'" :controls="controls" :thumbnail="file.thumbnail" :src="assetSrc"/>
    <span v-else>
      Unsupported File Type
    </span>

    <div v-if="fileType!== 'other' && showFileType" class="file-type-tag" :class="[fileType,file.fileSuffix]">{{ suffixName }}</div>
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

    &.encoded_image.raw {
      background: rgb(181, 19, 199);
    }
    &.encoded_image.psd {
      background: rgb(0,30,54);
    }
  }
}

</style>
