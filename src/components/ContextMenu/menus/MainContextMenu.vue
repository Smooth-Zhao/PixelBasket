<script setup lang="ts">
import ContextMenuItem from "../components/ContextMenuItem.vue";
import ContextMenuGroup from "../components/ContextMenuGroup.vue";
import ContextMenu from "../index.vue"
import {h, ref} from "vue";
import Basket from "../../../entities/Basket.ts";
import {invoke} from "@tauri-apps/api";
import BasketEditor from "../../BasketEditor.vue";
import {useMessage, useModal} from "naive-ui";
import {IContextMenuProps} from "../createContextMenu.ts";
import useBasket from "../../../hooks/useBasket.ts";
import {NIcon} from "naive-ui";
import {Checkmark20Regular} from "@vicons/fluent";
import useFolder from "../../../hooks/useFolder.ts";
import useContentBrowser from "../../../hooks/useContentBrowser.ts";

defineProps<IContextMenuProps<any>>()

const modal = useModal()
const message = useMessage()
const basket = ref(new Basket())
const onCreateBasket = (): void => {
  basket.value = new Basket()
  modal.create({
    title: '创建篮子',
    style: {
      width: "600px"
    },
    positiveText: "保存",
    async onPositiveClick() {
      if (!basket.value.name) {
        message.error("请输入篮子名称")
        return false
      }
      if (basket.value.directories.size <= 0) {
        message.error("选择关联文件夹")
        return false
      }
      await invoke("create_basket", {
        basket: {
          name: basket.value.name,
          directories: Array.from(basket.value.directories)
        }
      })
    },
    showIcon: false,
    maskClosable: false,
    content: () => h(BasketEditor, {
      basket: basket.value
    }),
    preset: 'dialog'
  })
}
const {baskets, currentBasket} = useBasket()
const folder = useFolder()
const contentBrowser = useContentBrowser()
const handleSwitchBasket = async (item: Basket) => {
  currentBasket.value = item
  await folder.load(item.id)
  await contentBrowser.load(folder.folderTree.value[0].path)
}
</script>

<template>
  <context-menu v-bind="$props">
    <context-menu-group>
      <context-menu-item shortcut="Alt + A">
        篮子
        <template #sub-menu>
          <div class="context-menu">
            <context-menu-group>
              <context-menu-item shortcut="Alt + F5" @click="onCreateBasket">
                创建篮子
              </context-menu-item>
            </context-menu-group>

            <context-menu-group v-if="baskets.length > 0">
              <context-menu-item
                :disabled="currentBasket?.id === item.id"
                @click="handleSwitchBasket(item)"
                v-for="item in baskets"
              >
                <template #prefix v-if="currentBasket?.id === item.id">
                  <n-icon size="14">
                    <Checkmark20Regular/>
                  </n-icon>
                </template>
                {{ item.name }}
              </context-menu-item>
            </context-menu-group>
          </div>
        </template>
      </context-menu-item>
    </context-menu-group>
  </context-menu>
</template>

<style scoped lang="scss"></style>
