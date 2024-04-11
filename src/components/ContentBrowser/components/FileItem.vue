<script setup lang="ts">
import {NEllipsis} from "naive-ui";
import useFileContextMenu from "../../../hooks/useFileContextMenu.ts";
import {useElementVisibility} from "@vueuse/core";
import {ref} from "vue";
import FilePreview from "./FilePreview.vue";
import useSelection from "../../../hooks/useSelection.ts";

const props = defineProps<{
  src: string
}>()
const el = ref()
const {trigger} = useFileContextMenu()
const visibility = useElementVisibility(el)

const {items} = useSelection()
const handleMouse = (e:MouseEvent) => {
  items.value.clear()
  items.value.add(props.src)
  trigger(e)
}
</script>

<template>
  <div class="file-item" @contextmenu.stop.prevent="handleMouse" ref="el">
    <div class="cover">
      <file-preview show-file-type v-if="visibility" :src="src"/>
    </div>
    <div class="info">
      <n-ellipsis style="max-width:100%">
        {{ decodeURIComponent(src).substring(decodeURIComponent(src).lastIndexOf("\\") + 1) }}
      </n-ellipsis>
      <span class="resolution">4096 * 1080</span>
    </div>
  </div>
</template>

<style scoped lang="scss">
.file-item {
  overflow: hidden;

  .cover {
    width: 100%;
    aspect-ratio: 1/1;
    //border-radius: 8px;
    overflow: hidden;
  }

  .info {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 16px;
    overflow: hidden;
    width: 100%;

    span {
      width: auto;
      max-width: 100%;
    }

    .file-name {
      color: var(--color-light-1);
      padding: 0 4px;
      text-align: center;
    }

    .resolution {
      font-size: 12px;
    }
  }

  &.selected {
    .cover {
      position: relative;
    }

    .cover::after {
      content: "";
      position: absolute;
      width: calc(100% - 4px);
      height: calc(100% - 4px);
      left: 0;
      top: 0;
      border: solid 2px #12b444;
      //border-radius: 8px;
      pointer-events: none;
    }

    .file-name {
      background-color: #12b444;
    }
  }
}
</style>
