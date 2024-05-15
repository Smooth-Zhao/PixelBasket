<script setup lang="ts">
import Layout from "../components/Layout.vue";
import WindowAction from "../components/WindowAction.vue";
import useBasket from "../hooks/useBasket.ts";
import useFolder from "../hooks/useFolder.ts";
import {invoke} from "@tauri-apps/api";

const basket = useBasket()
basket.init().then(() => {
  if (basket.baskets.value.length <= 0){
    // TODO：弹出创建篮子引导
  }else{
    useFolder().load(basket.baskets.value[0].id)
  }
})
invoke('run_task')
</script>

<template>
  <Layout/>
  <window-action/>
</template>

<style scoped lang="scss">

</style>
