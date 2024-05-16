import {ref} from "vue";
import PBFile from "../entities/PBFile.ts";
import {invoke} from "@tauri-apps/api";

const files = ref<PBFile[]>([])

const useContentBrowser = () => {
  return {
    load,
    files,
    isLoadChildren
  }
}
const isLoadChildren = ref<boolean>(false)
const load = async (path: string) => {
  files.value = await invoke<PBFile[]>("get_metadata_like_path", {path, like: isLoadChildren.value})
}

export default useContentBrowser
