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

export type FileType = "image" | "video" | "audio" | "document" | "other"
export const getFileType = (filename: string):FileType => {
  if (['AVIF', 'BMP', 'DDS', 'FARBFELD', 'GIF', 'HDR', 'ICO', 'JPG', 'JPEG', 'EXR', 'PNG', 'PNM', 'QOI', 'TGA', 'TIFF', 'WEBP'].includes(filename)) {
    return "image"
  } else if (["MP4","MOV"].includes(filename)) {
    return "video"
  }
  return "other"
}
