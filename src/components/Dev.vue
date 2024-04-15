<script setup lang="ts">
import {NFloatButtonGroup, NFloatButton, NIcon} from "naive-ui"
import {Bug20Filled} from "@vicons/fluent"
import useTask from "../hooks/useTask.ts";
import TaskEvent from "../entities/TaskEvent.ts";
import {hide} from "@tauri-apps/api/app";
import useContentBrowser from "../hooks/useContentBrowser.ts";
import {invoke} from "@tauri-apps/api";

const task = useTask()
const {readDirectory} = useContentBrowser()
const handleAddTask = () => {
  const testTask = {
    type: "task" + task.tasks.length,
    stage: "task_start",
    progress: 1,
    data: {}
  } as TaskEvent
  task.setTask(testTask)
  setInterval(() => {
    task.setTask({
      type: testTask.type,
      stage: "task_start",
      progress: testTask.progress++,
      data: {}
    })
  }, 1000)
}

const testApi = () => {
  invoke("get_metadata", {}).then(res => {
    console.log(res)
  })
}

</script>

<template>
  <n-float-button menu-trigger="hover" :right="20" :bottom="20" shape="square" position="absolute">
    <n-icon>
      <Bug20Filled></Bug20Filled>
    </n-icon>
    <template #menu>
      <n-float-button @click="handleAddTask">
        任务
      </n-float-button>
      <n-float-button @click="readDirectory">
        读取
      </n-float-button>
      <n-float-button @click="testApi">
        读取
      </n-float-button>
    </template>
  </n-float-button>
</template>

<style scoped lang="scss">

</style>
