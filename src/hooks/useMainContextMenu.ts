import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
import MainContextMenu from "../components/ContextMenu/menus/MainContextMenu.vue";
import {ref} from "vue";
const useMainContextMenu = () => {
  return createContextMenu("main", MainContextMenu, ref({}))
}


export default useMainContextMenu;
