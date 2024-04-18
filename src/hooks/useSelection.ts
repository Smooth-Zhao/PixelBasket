import {ref} from "vue";
import PBFile from "../entities/PBFile.ts";

const selectItems = ref<Set<PBFile>>(new Set())
const useSelection = () => {
  return {
    items: selectItems,
  }
}
export default useSelection;
