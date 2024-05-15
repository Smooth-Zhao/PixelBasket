import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
import FileContextMenu from "../components/ContextMenu/menus/FileContextMenu.vue";
import { ref} from "vue";

const useFileContextMenu = () => {
  return createContextMenu("file", FileContextMenu, ref({}))
}

export default useFileContextMenu;
