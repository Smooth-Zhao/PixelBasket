import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
import {useMessage, useModal} from "naive-ui"
import {h, ref} from "vue"
import {ModalApiInjection} from "naive-ui/es/modal/src/ModalProvider";
import BasketEditor from "../components/BasketEditor.vue";
import {invoke} from "@tauri-apps/api";
import Basket from "../entities/Basket.ts";
import {MessageApiInjection} from "naive-ui/es/message/src/MessageProvider";
const useMainContextMenu = () => {
  const modal = useModal()
  const message = useMessage()
  return createContextMenu({
    menu: [
      [
        {
          key: "basket",
          label: "篮子",
          shortcut: "Alt + A",
          children: [
            [{
              key: "basket.create",
              label: "创建篮子",
              handler() {
                onCreateBasket(modal,message);
              },
            }, {
              key: "basket.create",
              label: "创建篮子"
            }, {
              key: "basket.create",
              label: "创建篮子"
            }],
            [{
              key: "basket.create",
              label: "创建篮子"
            }, {
              key: "basket.create",
              label: "创建篮子"
            }, {
              key: "basket.create",
              label: "创建篮子"
            }]
          ]
        }
      ]
    ]
  })
}

const onCreateBasket = (modal: ModalApiInjection,message:MessageApiInjection): void => {
  const basket = ref(new Basket())
  modal.create({
    title: '创建篮子',
    style: {
      width: "600px"
    },
    positiveText: "保存",
    async onPositiveClick() {
      if (!basket.value.name){
        message.error("请输入篮子名称")
        return false
      }
      if (basket.value.directories.size <= 0){
        message.error("选择关联文件夹")
        return false
      }
      await invoke("create_basket", {
        basket: {
          name: basket.value.name,
          directories:Array.from(basket.value.directories)
        }
      })
    },
    showIcon: false,
    maskClosable: false,
    content: () => h(BasketEditor, {
      basket:basket.value
    }),
    preset: 'dialog'
  })
  // watch(basket, () => console.log(basket),{deep:true})
}
export default useMainContextMenu;
