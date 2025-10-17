import { createApp } from "vue";
import App from "./App.vue";
import "./styles/theme.css";
import { configureDayjs } from "./utils/dayjs-config";

configureDayjs().then(() => {
  createApp(App).mount("#app");
});
