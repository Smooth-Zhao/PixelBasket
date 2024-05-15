<script setup lang="ts">

import Layout from "../components/Layout.vue";
import WindowAction from "../components/WindowAction.vue";
import useBasket from "../hooks/useBasket.ts";
import useFolder from "../hooks/useFolder.ts";
import useFileContextMenu from "../hooks/useFileContextMenu.ts";
const basket = useBasket()
basket.init().then(()=>{
 useFolder().init(basket.basket.value[0].id)
})
useFileContextMenu()
</script>

<template>
  <Layout/>
  <window-action/>
</template>

<style scoped lang="scss">

</style>
