import {invoke} from "@tauri-apps/api";
import {arrayToTree} from "../utils";
import {ref} from "vue";

interface IFolder {
  id: string,
  pid: string,
  name: string,
  path: string
}

const folderTree = ref<Array<IFolder & { children: IFolder[] }>>([])
const useFolder = () => {

  return {
    load,
    folderTree
  }
}
const load = async (id: string) => {
  const result = await invoke<IFolder[]>("get_folder", {id})
  folderTree.value = arrayToTree(result, "pid")
}
export default useFolder
