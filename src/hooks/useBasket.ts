import {invoke} from "@tauri-apps/api";
import {ref} from "vue";
import Basket from "../entities/Basket.ts";
const baskets  = ref<Basket[]>([])
const currentBasket = ref<Basket>()

const useBasket = function () {
  return{
    init,
    baskets,
    currentBasket
  }
}

const init = async () => {
  baskets.value = await invoke<Basket[]>("get_basket", {})
}

export default useBasket
