import { createApp } from "vue";
import "./styles.scss";
import App from "./App.vue";
import initListener from "./listeners";

initListener()

createApp(App).mount("#app");
