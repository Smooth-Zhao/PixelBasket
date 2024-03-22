import {ref, render, h} from "vue";
import ContextMenu from "./index.vue"

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
const createContextMenu = (option: ICreateContextMenuOption) => {
  render(h(ContextMenu), document.body)

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
  return () => ({
    trigger
  })
}
export default createContextMenu
