<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import useTask from "../hooks/useTask.ts";
import {NProgress} from "naive-ui";

onMounted(() => {
  setInterval(() => {
    if (!tasks.length) return
    currentTaskIndex.value = (currentTaskIndex.value + 1) % tasks.length
  },5000)
})
const {tasks} = useTask()
const currentTaskIndex = ref(0)
const currentTask = computed(() => tasks[currentTaskIndex.value])

</script>

<template>
  <div class="content-footer">
    <div class="task-container">
      <template v-if="tasks.length > 0">
        <transition>
          <div class="task-item" :key="currentTask.type">
            <n-progress
              type="line"
              :percentage="currentTask.progress % 99 ^ 0"
              rail-style="height:4px"
            >
              {{ currentTask.type }} {{currentTask.progress % 99 ^ 0}}%
            </n-progress>
          </div>
        </transition>
      </template>
    </div>
  </div>
</template>

<style scoped lang="scss">
.content-footer {
  flex: 0 0 30px;
  transition: height .2s ease;
  overflow: hidden;
  border-top: solid 1px var(--color-dark-3);
  display: flex;
  align-items: center;
  padding: 0 16px;
  .task-container{
    flex: 0 0 300px;
    height: 100%;
    position: relative;
    .task-item{
      position: absolute;
      width: 100%;
      height: 100%;
      left: 0;
      top: 0;
      display: flex;
      align-items: center;

    }
    .v-enter-active,
    .v-leave-active {
      transition: all .2s ease-out;
    }
    .v-enter-from,
    .v-leave-to {
      opacity: 0;
    }
    .v-enter-from{
      transform: translateY(100%);
    }
    .v-enter-to{
      transform: translateY(0);
    }
  }
}
</style>
