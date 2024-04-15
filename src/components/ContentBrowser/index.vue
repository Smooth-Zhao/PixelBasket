<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import {NScrollbar} from "naive-ui"
import FileItem from "./components/FileItem.vue";
import useSelection from "../../hooks/useSelection.ts";
import useContentBrowser from "../../hooks/useContentBrowser.ts";

const columnNumber = ref(4)

const {items: selectItems} = useSelection()
const {files,loadLocalStorage} = useContentBrowser()

const handleSelect = (e: PointerEvent, src: string) => {
  if (e.shiftKey) {
    selectFile(src, true)
  } else if (e.ctrlKey) {
    selectFile(src)
  } else {
    selectItems.value.clear()
    selectFile(src)
  }
}

const selectFile = (current: any, addition = false) => {
  if (addition) {
    const arr = Array.from(selectItems.value)
    const firstIndex = files.value.findIndex(v=>v.src === arr.at(0));
    const currentIndex = files.value.findIndex(v=>v.src === current);
    selectItems.value.clear()
    files.value.slice(
      Math.min(firstIndex, currentIndex),
      Math.max(firstIndex, currentIndex) + 1
    ).forEach(v => selectItems.value.add(v.src))
  } else {
    selectItems.value.add(current)
  }
}

const handleSelectNone = () => {
  selectItems.value.clear()
}

const handleKeyUp = (e: KeyboardEvent) => {
  if (e.key.startsWith("Arrow")) {
    handleArrowKey(e.key.slice(5).toLowerCase())
  }
}

//TODO arrow key select
const handleArrowKey = (key: string) => {
  switch (key) {
    case "up": {

      break
    }
    case "down": {

      break
    }
    case "left": {

      break
    }
    case "right": {

      break
    }
  }
}
onMounted(() => {
  loadLocalStorage()
  document.removeEventListener("keyup", handleKeyUp)
  document.addEventListener("keyup", handleKeyUp)
})
onUnmounted(() => {
  document.removeEventListener("keyup", handleKeyUp)
})
</script>

<template>
<!--  <n-slider v-model:value="columnNumber" :step="1" :min="1" :max="10"/>-->
  <n-scrollbar>
    <div class="content-browser" @click="handleSelectNone" @keydown="handleKeyUp">
      <file-item
        v-for="item in files"
        :src="item.src"
        :class="{ 'selected': selectItems.has(item.src) }"
        @click.stop="handleSelect($event,item.src)"
      />
    </div>
  </n-scrollbar>
</template>

<style scoped lang="scss">
.content-browser {
  min-height: 100%;
  display: grid;
  grid-template-columns: repeat(v-bind(columnNumber), 1fr);
  gap: 16px;
  align-content: flex-start;
  transition: all .2s ease;
  padding: 16px;
}
</style>
