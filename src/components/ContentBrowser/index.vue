<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import {NSlider, NButton, NScrollbar} from "naive-ui"
import {open} from "@tauri-apps/api/dialog";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import FileItem from "./components/FileItem.vue";
import useSelection from "../../hooks/useSelection.ts";
import {invoke} from "@tauri-apps/api";

const columnNumber = ref(4)
const images = ref<string[]>([])
const {items: selectItems} = useSelection()

const handleTest = async () => {
  images.value = []
  const path = await open({
    directory: true
  }) as string
  const result = await invoke("get_directory_files",{path}) as string

  (JSON.parse(result) as string[]).forEach(filePath => {
    const url = convertFileSrc(filePath)
    images.value.push(url)
  })
  localStorage.setItem("images", JSON.stringify(images.value))
}

const handleSelect = (e: PointerEvent, name: string) => {
  if (e.shiftKey) {
    selectFile(name, true)
  } else if (e.ctrlKey) {
    selectFile(name)
  } else {
    selectItems.value.clear()
    selectFile(name)
  }
}

const selectFile = (current: any, addition = false) => {
  if (addition) {
    const arr = Array.from(selectItems.value)
    const firstIndex = images.value.indexOf(arr.at(0));
    const currentIndex = images.value.indexOf(current);
    selectItems.value.clear()
    images.value.slice(
      Math.min(firstIndex, currentIndex),
      Math.max(firstIndex, currentIndex) + 1
    ).forEach(v => selectItems.value.add(v))
  } else {
    selectItems.value.add(current)
  }
}

const refresh = () => {
  images.value = JSON.parse(localStorage.getItem("images")) || []
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
  refresh()
  document.removeEventListener("keyup", handleKeyUp)
  document.addEventListener("keyup", handleKeyUp)
})
onUnmounted(() => {
  document.removeEventListener("keyup", handleKeyUp)
})
</script>

<template>
  <n-slider v-model:value="columnNumber" :step="1" :min="1" :max="10"/>

  <n-scrollbar>
    <div class="content-browser" @click="handleSelectNone" @keydown="handleKeyUp">
      <n-button @click="handleTest">test</n-button>
      <file-item
        v-for="item in images"
        :src="item"
        :class="{ 'selected': selectItems.has(item) }"
        @click.stop="handleSelect($event,item)"
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
