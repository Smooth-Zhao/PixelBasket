import {os, shell} from "@tauri-apps/api";
import PBFile from "../entities/PBFile.ts";
import {WebviewWindow} from "@tauri-apps/api/window";

export const isFunction = <T>(val: unknown): val is (...args: any) => T => {
  return Object.prototype.toString.call(val) === '[object Function]'
}
/**
 * 节流
 */
export const throttle = <T extends (...args: any) => any>(func: T, wait: number) => {
  let timeout: number | null
  let lastArgs: Parameters<T>
  let lastThis: ThisParameterType<T>
  return function (this: ThisParameterType<T>, ...args: Parameters<T>) {
    if (!timeout) {
      timeout = setTimeout(() => {
        timeout = null
        func.apply(lastThis, lastArgs)
      }, wait)
    }
    lastArgs = args
    lastThis = this
  } as T
}

export type FileType = "image" | "encoded_image" | "video" | "audio" | "psd" | "other"
export const getFileType = (filename: string): FileType => {
  filename = filename.toUpperCase()
  if (['AVIF', 'BMP', 'DDS', 'FARBFELD', 'GIF', 'HDR', 'ICO', 'JPG', 'JPEG', 'EXR', 'PNG', 'PNM', 'QOI', 'TGA', 'TIFF', 'WEBP'].includes(filename)) {
    return "image"
  } else if (["NEF", "PSD"].includes(filename)) {
    return "encoded_image"
  } else if (["MP4", "MOV", "WEBM"].includes(filename)) {
    return "video"
  }
  return "other"
}
export const compareType = (suffix: string, type: FileType | FileType[]): boolean => {
  if (Array.isArray(type)) {
    return type.includes(getFileType(suffix))
  }
  return getFileType(suffix) === type
}
//covert bytes to size
export const bytesToSize = (bytes: number): string => {

  if (bytes === 0) return '0 B';

  const k = 1024;
  const dm = 2;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

export const openFile = async (file: PBFile) => {
  if (!compareType(file.fileSuffix, "image")) {
    openFileWithLocalProgram(file.fullPath)
  } else {
    const fileWindow = WebviewWindow.getByLabel("file")
    if (fileWindow) {
      fileWindow.emit("update_file", {id: file.id})
      fileWindow.setFocus()
    } else {
      const webview = new WebviewWindow("file", {
        url: `/file/${file.id}`,
        decorations: false,
        title: "查看文件",
        visible: false
      });
      webview.once("page_loaded", () => {
        webview.show()
      })
    }
  }
}

export const openFileWithLocalProgram = async (src: string) => {
  let command: shell.Command | null = null;
  switch (await os.type()) {
    case "Windows_NT": {
      command = new shell.Command("cmd", ["/C", "start", "", src])
      break
    }

    case "Darwin": {
      command = new shell.Command("open", [src])
      break
    }

    case "Linux": {
      command = new shell.Command("xdg-open", [src])
      break
    }
  }
  command?.execute().then(res => {
    console.log(res)
  }).catch(e => {
    console.error(e)
  })
}

/**
 * 将数组转换为树形结构
 * @param arr
 * @param key
 */
export function arrayToTree<T extends { id: string }>(arr: T[], key: keyof T) {
  const roots:(
    T & {
      children: T[]
    }
  )[] = [];

  const lookup:{
    [id : string]: T & {
      children: T[]
    }
  } = {};

  // 创建一个lookup对象来快速查找节点
  arr.forEach(item => {
    lookup[item.id] = {
      children: [],
      ...item
    };
  });
  // 将节点放入正确的位置
  arr.forEach(item => {
    if (item[key] === "0") {
      roots.push(lookup[item.id]);
    } else {
      lookup[item[key] as string].children.push(lookup[item.id]);
    }
  });

  return roots;
}
