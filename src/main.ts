import { createApp, watch } from "vue";
import App from "./App.vue";
import "./styles/theme.scss";
import { configureDayjs, setDayjsLocale } from "./utils/dayjs-config";
import { i18n, setI18nLocale } from "./i18n";
import { useTheme } from "./stores/themeStore";
import { useLocale } from "./stores/localeStore";
import { useUserSelection } from "./stores/userStore";

async function initializeApp() {
  try {
    // Day.js プラグイン設定
    configureDayjs();

    // テーマ初期化
    const { initTheme } = useTheme();
    initTheme();

    // ロケール初期化
    const { locale, initLocale } = useLocale();
    await initLocale();

    // ユーザー選択初期化
    const { initSelectedUser } = useUserSelection();
    initSelectedUser();

    // ロケール変更時の副作用
    watch(locale, (newLocale) => {
      setI18nLocale(newLocale);
      setDayjsLocale(newLocale);
      document.documentElement.lang = newLocale;
    }, { immediate: true });

    // Vue アプリ作成とマウント
    const app = createApp(App);
    app.use(i18n);
    app.mount("#app");
  } catch (error) {
    console.error('Failed to initialize app:', error);
  }
}

initializeApp();
