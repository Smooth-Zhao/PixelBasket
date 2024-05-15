import {ref, Component, Ref, shallowReactive} from "vue";

export interface IContextMenuProps<T>{
  payload:T,
  show:boolean,
  position:{
    x:number,
    y:number,
  }
}
export const contextMenuMap = shallowReactive(new Map<string,{
  component:Component,
  props:Ref<IContextMenuProps<any>>
}>())

/**
 *
 * @param name
 * @param component
 * @param initialProps
 */
const createContextMenu = function <T extends Ref>(name:string,component:Component,initialProps:T) {
  let componentProps:Ref<IContextMenuProps<T>>;
  if (contextMenuMap.has(name)){
    componentProps = contextMenuMap.get(name)!.props
  }else{
    componentProps = ref<IContextMenuProps<T>>({
      payload:initialProps.value,
      show:false,
      position:{
        x:0,
        y:0,
      }
    })

    contextMenuMap.set(name,{
      component,
      props:componentProps
    })
  }
  return {
    props:componentProps,
    trigger(e: MouseEvent){
      componentProps.value.position.x =  e.clientX
      componentProps.value.position.y = e.clientY
      componentProps.value.show = true
    }
  }
}
export default createContextMenu
