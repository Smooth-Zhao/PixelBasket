import {ref} from "vue";
import PBFile from "../entities/PBFile.ts";
import {open} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/tauri";

const files = ref<PBFile[]>([])

const useContentBrowser = () => {

  return {
    readDirectory,
    loadLocalStorage,
    files
  }
}
const loadLocalStorage = () => {
  files.value = JSON.parse(<string>localStorage.getItem("files")) || []
}

const readDirectory = async () => {
  files.value = []
  const path = await open({
    directory: true
  }) as string
  const result = await invoke("get_directory_files",{path}) as string

  (JSON.parse(result) as string[]).forEach(filePath => {
    const src = convertFileSrc(filePath)
    files.value.push({
      src,
      name:"xxx",
    })
  })
  localStorage.setItem("files", JSON.stringify(files.value))
}

export default useContentBrowser
