export const isFunction = <T>(val: unknown): val is (...args: any) => T => {
  return Object.prototype.toString.call(val) === '[object Function]'
}
