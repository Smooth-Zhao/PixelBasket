<script setup lang="ts">
import {Menu} from "@vicons/ionicons5";
import {NIcon} from "naive-ui";
import useMainContextMenu from "../../../ContextMenu/hooks/useMainContextMenu.ts";

const { trigger } = useMainContextMenu()

</script>

<template>
  <n-icon size="18" @click="trigger">
    <Menu/>
  </n-icon>
</template>

<style scoped lang="scss">

</style>
