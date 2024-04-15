<script setup lang="ts">
import {computed, onMounted} from "vue";
import {emit, listen} from "@tauri-apps/api/event";
import {useRouter} from "vue-router";
import FileWindowHeader from "../components/FileWindowHeader.vue";
import {ImageViewer} from "../components/FileViewer";
import {convertFileSrc} from "@tauri-apps/api/tauri";

const props = defineProps<{
  src: string
}>()

const router = useRouter()
onMounted(() => {
  setTimeout(() => emit("page_loaded"), 200)
  listen<{
    file:string
  }>("update_file", function (e) {
    router.replace({path: `/file/${e.payload.file}`})
  })
})
const assetSrc = computed(()=>convertFileSrc(props.src))
</script>

<template>
  <div class="file-window">
    <file-window-header/>
    <div class="file-content">
      <image-viewer controls :src="assetSrc"/>
    </div>
  </div>
</template>

<style scoped lang="scss">
.file-window{
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  border: solid 1px var(--color-dark-3);
  .file-content{
    height: 0;
    flex: 1 1 auto;
    padding: 16px;
    overflow: hidden;
  }
}
</style>
