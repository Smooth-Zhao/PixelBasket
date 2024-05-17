<script setup lang="ts">
import {NMessageProvider, NConfigProvider, NModalProvider,NDialogProvider} from "naive-ui";
import {darkTheme} from 'naive-ui'
import ContextMenuProvider from "./components/ContextMenu/ContextMenuProvider.vue";
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-modal-provider>
      <n-message-provider>
        <n-dialog-provider>
          <context-menu-provider>
            <router-view/>
          </context-menu-provider>
        </n-dialog-provider>
      </n-message-provider>
    </n-modal-provider>
  </n-config-provider>
</template>

