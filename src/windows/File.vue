<script setup lang="ts">
import {onMounted, ref, watch} from "vue";
import {emit, listen} from "@tauri-apps/api/event";
import {useRouter} from "vue-router";
import FileWindowHeader from "../components/FileWindowHeader.vue";
import {ImageViewer} from "../components/FileViewer";
import PBFile from "../entities/PBFile.ts";
import {invoke} from "@tauri-apps/api";

const props = defineProps<{
  id: string
}>()

const file = ref<PBFile>()
const router = useRouter()

watch(() => props.id, async () => {
  file.value = await invoke<PBFile>("get_metadata_by_id", {id: props.id})
},{
  immediate: true
})

onMounted(() => {
  setTimeout(() => emit("page_loaded"), 200)
  listen<{
    id: string
  }>("update_file", function (e) {
    router.replace({path: `/file/${e.payload.id}`})
  })
})


</script>

<template>
  <div class="file-window">
    <file-window-header/>
    <div class="file-content" v-if="file">
      <image-viewer controls :file="file"/>
    </div>
  </div>
</template>

<style scoped lang="scss">
.file-window {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  border: solid 1px var(--color-dark-3);

  .file-content {
    height: 0;
    flex: 1 1 auto;
    padding: 16px;
    overflow: hidden;
  }
}
</style>
