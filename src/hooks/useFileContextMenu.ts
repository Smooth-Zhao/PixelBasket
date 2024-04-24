import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
import {WebviewWindow} from "@tauri-apps/api/window";
import useSelection from "./useSelection.ts";

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
            const fileWindow = WebviewWindow.getByLabel("file")
            if (fileWindow) {
              fileWindow.emit("update_file", {id: file.id})
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
          },
        }
      ]
    ]
  })
}

export default useFileContextMenu;
