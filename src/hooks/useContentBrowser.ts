import {ref} from "vue";
import PBFile from "../entities/PBFile.ts";
import {invoke} from "@tauri-apps/api";

const files = ref<PBFile[]>([])

const useContentBrowser = () => {
  return {
    load,
    files
  }
}

const load = async (path:string,like=true) => {
  files.value = await invoke<PBFile[]>("get_metadata_like_path",{path,like})
}

export default useContentBrowser
