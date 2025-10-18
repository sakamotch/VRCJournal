import { createApp, watch } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./styles/theme.scss";
import { configureDayjs, setDayjsLocale } from "./utils/dayjs-config";
import { i18n, setI18nLocale } from "./i18n";
import { useThemeStore } from "./stores/themeStore";
import { useLocaleStore } from "./stores/localeStore";
import { useUserStore } from "./stores/userStore";
import { storeToRefs } from "pinia";

async function initializeApp() {
  try {
    // Day.js プラグイン設定
    configureDayjs();

    // Vue アプリ作成
    const app = createApp(App);

    // プラグイン登録
    const pinia = createPinia();
    app.use(pinia);
    app.use(i18n);

    // ストア初期化
    const themeStore = useThemeStore();
    themeStore.initTheme();

    const localeStore = useLocaleStore();
    await localeStore.initLocale();

    const userStore = useUserStore();
    userStore.initSelectedUser();

    // ロケール変更時の副作用
    const { locale } = storeToRefs(localeStore);
    watch(locale, (newLocale) => {
      setI18nLocale(newLocale);
      setDayjsLocale(newLocale);
      document.documentElement.lang = newLocale;
    }, { immediate: true });

    // Vue アプリマウント
    app.mount("#app");
  } catch (error) {
    console.error('Failed to initialize app:', error);
  }
}

initializeApp();
