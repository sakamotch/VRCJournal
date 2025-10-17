import { createApp } from "vue";
import App from "./App.vue";
import "./styles/theme.css";
import { configureDayjs } from "./utils/dayjs-config";
import { i18n, getInitialLocale, setLocale } from "./i18n";

async function initializeApp() {
  const locale = await getInitialLocale();
  configureDayjs();
  setLocale(locale);

  const app = createApp(App);
  app.use(i18n);
  app.mount("#app");
}

initializeApp();
