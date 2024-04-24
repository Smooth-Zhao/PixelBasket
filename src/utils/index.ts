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
export const getFileType = (filename: string):FileType => {
  filename = filename.toUpperCase()
  console.log(filename)
  if (['AVIF', 'BMP', 'DDS', 'FARBFELD', 'GIF', 'HDR', 'ICO', 'JPG', 'JPEG', 'EXR', 'PNG', 'PNM', 'QOI', 'TGA', 'TIFF', 'WEBP'].includes(filename)) {
    return "image"
  } else if (["NEF","PSD"].includes(filename)) {
    return "encoded_image"
  }else if (["MP4","MOV"].includes(filename)) {
    return "video"
  }
  return "other"
}
export const compareType = (suffix:string,type: FileType | FileType[]): boolean => {
  if (Array.isArray(type)){
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
