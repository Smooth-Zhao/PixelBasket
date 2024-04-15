import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
import {useModal} from "naive-ui"
import {h, ref} from "vue"
import {ModalApiInjection} from "naive-ui/es/modal/src/ModalProvider";
import BasketEditor from "../components/BasketEditor.vue";
import {invoke} from "@tauri-apps/api";
import Basket from "../entities/Basket.ts";
const useMainContextMenu = () => {
  const modal = useModal()
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
                onCreateBasket(modal)
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

const onCreateBasket = (modal: ModalApiInjection): void => {
  const basket = ref(new Basket())
  modal.create({
    title: '创建篮子',
    style: {
      width: "600px"
    },
    positiveText: "保存",
    async onPositiveClick() {
      const result = await invoke("create_basket", {
        basket: {
          name: basket.value.name,
          directories:Array.from(basket.value.directories)
        }
      })
      alert(result)
      return false
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
