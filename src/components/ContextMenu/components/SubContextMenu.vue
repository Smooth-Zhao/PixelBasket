<script setup lang="ts">

import ContextMenuGroup from "./ContextMenuGroup.vue";
import ContextMenuItem from "./ContextMenuItem.vue";
import {IContextMenuGroup, IContextMenuItem} from "../createContextMenu.ts";
import SubContextMenu from "./SubContextMenu.vue"
import {NIcon} from "naive-ui";
import {CaretRight20Filled} from "@vicons/fluent";
import {isFunction} from "../../../utils";
import {inject} from "vue";
import {contextMenuHandlerKey} from "../../../utils/injectionKeys.ts";
defineProps<{
  data:IContextMenuGroup[]
}>()

const handlerExecute = inject(contextMenuHandlerKey,()=>{})

const handleClick = (item:IContextMenuItem)=>{
  if (!item.children && isFunction(item.handler)){
    item.handler.call(undefined)
    handlerExecute()
  }
}

</script>

<template>
  <div class="context-menu">
    <context-menu-group v-for="group in data">
      <context-menu-item v-for="item in group" @click="handleClick(item)">
        {{item.label}}
        <span v-if="item.shortcut" class="shortcut">
          {{ item.shortcut }}
        </span>
        <template v-if="Array.isArray(item.children)">
          <n-icon size="12" class="icon-more">
            <CaretRight20Filled/>
          </n-icon>
          <sub-context-menu :data="item.children"/>
        </template>
      </context-menu-item>
    </context-menu-group>
  </div>
</template>

<style scoped lang="scss">
.context-menu{
  left: 100%;
  top: 0;
  display: none;
}
</style>
