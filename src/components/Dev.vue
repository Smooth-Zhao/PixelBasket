<script setup lang="ts">
import {NFloatButton, NIcon} from "naive-ui"
import {Bug20Filled} from "@vicons/fluent"
import useTask from "../hooks/useTask.ts";
import TaskEvent from "../entities/TaskEvent.ts";
import {invoke} from "@tauri-apps/api";
import useFolder from "../hooks/useFolder.ts";
import useBasket from "../hooks/useBasket.ts";

const basket = useBasket()
const load = () => {
  basket.init().then(() => {
    if (basket.baskets.value.length <= 0) {
      // TODO：弹出创建篮子引导
    } else {
      useFolder().load(basket.baskets.value[0].id)
    }
  })
}

const task = useTask()
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
const runTask = ()=>{

  invoke('run_task')
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
      <n-float-button @click="runTask">
        扫描
      </n-float-button>
    </template>
  </n-float-button>
</template>

<style scoped lang="scss">

</style>
