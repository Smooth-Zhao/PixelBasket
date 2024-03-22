<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import {invoke} from "@tauri-apps/api/tauri";

// invoke("get_directory_tree", { path: "E:\\照片" }).then(res=>{
//     console.log(JSON.parse(res as string))
// });
import Layout from "./components/Layout.vue";
import {NMessageProvider,NConfigProvider} from "naive-ui";
import {darkTheme} from 'naive-ui'
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <Layout/>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>

</style>
