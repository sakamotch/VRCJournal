import { createApp } from "vue";
import { watch } from "vue";
import App from "./App.vue";
import "./styles/theme.css";
import { configureDayjs } from "./utils/dayjs-config";
import { i18n } from "./i18n";
import { useLocale } from "./stores/localeStore";

async function initializeApp() {
  configureDayjs();

  // ロケール初期化
  const { locale, initLocale } = useLocale();
  await initLocale();

  // localeStore の変更を vue-i18n に同期
  watch(locale, (newLocale) => {
    i18n.global.locale.value = newLocale;
  }, { immediate: true });

  const app = createApp(App);
  app.use(i18n);
  app.mount("#app");
}

initializeApp();
