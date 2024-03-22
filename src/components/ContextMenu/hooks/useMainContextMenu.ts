import createContextMenu from "../createContextMenu.ts";

const useMainContextMenu = createContextMenu({
  menu: [
    [
      {
        key: "basket",
        label: "篮子",
        shortcut: "Alt + A",
        children: [
          [{
            key: "basket.create",
            label: "创建篮子",
            handler() {
              console.log("222")
            },
          }, {
            key: "basket.create",
            label: "创建篮子"
          }, {
            key: "basket.create",
            label: "创建篮子"
          }],
          [{
            key: "basket.create",
            label: "创建篮子"
          }, {
            key: "basket.create",
            label: "创建篮子"
          }, {
            key: "basket.create",
            label: "创建篮子"
          }]
        ]
      }
    ]
  ]
})
export default useMainContextMenu;
