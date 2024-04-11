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
