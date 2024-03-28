import {ref, render, h} from "vue";
import ContextMenu from "./index.vue"
import {isFunction} from "../../utils";

export interface IContextMenuItem {
  key: string
  label: string
  shortcut?:string
  handler?: () => void
  children?: IContextMenuGroup[]
}
export type IContextMenuGroup = IContextMenuItem[]
interface ICreateContextMenuOption {
  menu: IContextMenuGroup[]
}

export const activeContextMenuData = ref<{
  option: ICreateContextMenuOption,
  position: {
    x: number,
    y: number
  }
}>()
const createContextMenu = (param: ICreateContextMenuOption | (() => ICreateContextMenuOption)) => {
  render(h(ContextMenu), document.body)
  let option:ICreateContextMenuOption;
  if (isFunction(param)){
    option = param()
  }else{
    option = param
  }

  const trigger = (e: MouseEvent) => {
    e.stopPropagation()
    e.preventDefault()
    activeContextMenuData.value = {
      option,
      position: {
        x: e.clientX,
        y: e.clientY
      }
    }
  }
  return {
    trigger
  }
}
export default createContextMenu
