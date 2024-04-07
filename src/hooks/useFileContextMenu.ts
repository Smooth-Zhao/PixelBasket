import createContextMenu from "../components/ContextMenu/createContextMenu.ts";
const useFileContextMenu = () => {
  return createContextMenu({
    menu: [
      [
        {
          key: "open",
          label: "打开",
          shortcut: "F5",
        }
      ]
    ]
  })
}

export default useFileContextMenu;
