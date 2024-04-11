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
            const file = encodeURIComponent(Array.from(selection.items.value)[0] as string);
            const fileWindow = WebviewWindow.getByLabel("file")
            console.log(fileWindow)
            if (fileWindow) {
              fileWindow.emit("update_file", {file})
            } else {
              const webview = new WebviewWindow("file", {
                url: `/file/${file}`,
                decorations:false,
                title: "查看文件",
                visible: false
              });
              webview.once("page_loaded", () => {
                console.log(webview)
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
