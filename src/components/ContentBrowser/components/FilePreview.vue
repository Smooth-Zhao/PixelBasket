<script setup lang="ts">
import {computed, ref} from "vue";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import VideoPreview from "./VideoPreview.vue";

const props = defineProps<{
  src: string,
  showFileType?: boolean
}>()

const assetSrc = computed(() => convertFileSrc(props.src))

const suffixName = computed(() => {
  return props.src?.split('.').pop()?.toUpperCase();
})

const fileType = computed(() => {
  const suffixName = props.src?.split('.').pop()?.toUpperCase();
  if (!suffixName) return;
  if (['AVIF', 'BMP', 'DDS', 'FARBFELD', 'GIF', 'HDR', 'ICO', 'JPG', 'JPEG', 'EXR', 'PNG', 'PNM', 'QOI', 'TGA', 'TIFF', 'WEBP'].includes(suffixName)) {
    return "image"
  } else if (["MP4"].includes(suffixName)) {
    return "video"
  }
  return "unsupported"
})

</script>

<template>
  <div class="file-preview">
    <img v-if="fileType === 'image'" :src="assetSrc" alt="">
    <video-preview  v-else-if="fileType === 'video'" :src="assetSrc"/>
    <span v-else>
      Unsupported File Type
    </span>

    <div v-if="fileType!== 'unsupported' && showFileType" class="file-type-tag" :class="fileType">{{ suffixName }}</div>
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

img {
  object-fit: contain;
  width: 100%;
  height: 100%;
  display: block;
}
</style>
