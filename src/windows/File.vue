<script setup lang="ts">
import {computed, onMounted} from "vue";
import {emit, listen} from "@tauri-apps/api/event";
import {useRouter} from "vue-router";
import FilePreview from "../components/ContentBrowser/components/FilePreview.vue";
import FileWindowHeader from "../components/FileWindowHeader.vue";

defineProps<{
  path: string
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
</script>

<template>
  <div class="file-window">
    <file-window-header/>
    <file-preview :src="path"/>
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
}
</style>
