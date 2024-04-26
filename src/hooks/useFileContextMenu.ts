import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
import useSelection from "./useSelection.ts";
import {openFile} from "../utils";

const useFileContextMenu = () => {
  const selection = useSelection();
  return createContextMenu({
    menu: [
      [
        {
          key: "open",
          label: "打开",
          shortcut: "F5",
          handler() {
            const file = Array.from(selection.items.value)[0];
            openFile(file)
          },
        }
      ]
    ]
  })
}

export default useFileContextMenu;
