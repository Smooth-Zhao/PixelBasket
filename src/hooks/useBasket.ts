import {invoke} from "@tauri-apps/api";
import {ref} from "vue";

interface Basket {
  id: string,
  name: string
}

const basket  = ref<Basket[]>([])

const useBasket = function () {

  return{
    init,
    basket
  }
}

const init = async () => {
  basket.value = await invoke<Basket[]>("get_basket", {})
  console.log(basket.value)
}

export default useBasket
