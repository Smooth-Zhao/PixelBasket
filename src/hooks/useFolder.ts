import {invoke} from "@tauri-apps/api";
import {arrayToTree} from "../utils";
import {ref} from "vue";

export interface IFolder {
  id: string,
  pid: string,
  name: string,
  path: string
}

const folderTree = ref<Array<IFolder & { children: IFolder[] }>>([])
const currentFolder = ref<string>("")
const useFolder = () => {

  return {
    load,
    folderTree,
    currentFolder
  }
}
const load = async (id: string) => {
  const result = await invoke<IFolder[]>("get_folder", {id})
  folderTree.value = arrayToTree(result, "pid")
}
export default useFolder
