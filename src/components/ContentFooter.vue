<script setup lang="ts">
import {NProgress} from "naive-ui"
import {onMounted, ref} from "vue";
const percent = ref(0)
onMounted(()=>{
  // setInterval(()=>{
  //   percent.value += 0.1
  // },1000 / 60)
})
</script>

<template>
  <div class="content-footer">
<!--    <n-progress type="line" :percentage="percent % 99 ^ 0"></n-progress>-->
  </div>
</template>

<style scoped lang="scss">
.content-footer {
  height: 40px;
  transition: height .2s ease;
  overflow: hidden;
  border-top: solid 1px var(--color-dark-3);
  display: flex;
  align-items: center;
  padding: 0 16px;
}
</style>
