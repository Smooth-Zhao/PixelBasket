import { createApp } from "vue";
import "./styles.scss";
import App from "./App.vue";
import initListener from "./listeners";
import router from "./router"
initListener()

createApp(App).use(router).mount("#app");
