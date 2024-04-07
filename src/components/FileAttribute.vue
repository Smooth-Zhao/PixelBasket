<script setup lang="ts">
import useSelection from "../hooks/useSelection.ts";
import {NScrollbar} from "naive-ui";

const {items} = useSelection()
</script>

<template>
  <n-scrollbar>
    <div class="file-attribute">
      <img style="width: 100%" :src="Array.from(items).at(-1)">
    </div>
  </n-scrollbar>
</template>

<style scoped lang="scss">
.file-attribute{
  height: 100%;
  padding: 56px 16px 16px;
}
</style>
