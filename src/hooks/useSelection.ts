import {ref} from "vue";

const selectItems = ref<Set<any>>(new Set())
const useSelection = () => {
  return {
    items: selectItems
  }
}
export default useSelection;
