<script setup lang="ts">
import {NMessageProvider, NConfigProvider, NModalProvider} from "naive-ui";
import {darkTheme} from 'naive-ui'
import ContextMenuProvider from "./components/ContextMenu/ContextMenuProvider.vue";
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-modal-provider>
      <n-message-provider>
        <context-menu-provider>
          <router-view/>
        </context-menu-provider>
      </n-message-provider>
    </n-modal-provider>
  </n-config-provider>
</template>

