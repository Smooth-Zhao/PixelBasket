<script setup lang="ts">
import {NInput, NButton, NIcon} from "naive-ui";
import {Add20Filled} from "@vicons/fluent"
import {open} from '@tauri-apps/api/dialog';
import Basket from "../entities/Basket.ts"
import {listen} from "@tauri-apps/api/event";
import {ref} from "vue";
const value = ref("")
const basket = defineModel<Basket>("basket", {
  default: new Basket()
})
listen("app",(e)=>{
  if (typeof e.payload === "string") {
    value.value = e.payload
  }
})
const handleAddFolder = async () => {
  const result = await open({
    directory: true
  })
  basket.value?.directories.add(result as string)
}
</script>

<template>
  <div class="basket-editor">
    <label>
      <span>篮子名称：{{value}}</span>
      <n-input v-model:value="basket.name" placeholder="输入篮子名称"></n-input>
    </label>
    <div class="folder-list">
      <div class="folder-list-header">
        <span>关联文件夹：</span>
        <n-button quaternary type="primary" @click="handleAddFolder">
          <n-icon>
            <Add20Filled/>
          </n-icon>
          添加文件夹
        </n-button>
      </div>
      <div v-for="item in basket.directories" class="folder-item">{{ item }}</div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.basket-editor {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.folder-list {
  .folder-list-header {
    display: flex;
    border-bottom: var(--color-gray-1) solid 1px;
    justify-content: space-between;
    align-items: center;
  }

  .folder-item {
    padding: 4px 0;
  }
}

</style>
