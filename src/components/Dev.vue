<script setup lang="ts">
import {NFloatButton, NIcon} from "naive-ui"
import {Bug20Filled} from "@vicons/fluent"
import useTask from "../hooks/useTask.ts";
import TaskEvent from "../entities/TaskEvent.ts";
import useContentBrowser from "../hooks/useContentBrowser.ts";

const task = useTask()
const {load} = useContentBrowser()
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
      <n-float-button @click="load">
        读取
      </n-float-button>
      <n-float-button>
        Raw
      </n-float-button>
    </template>
  </n-float-button>
</template>

<style scoped lang="scss">

</style>
