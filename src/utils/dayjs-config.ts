import dayjs from "dayjs";
import "dayjs/locale/ja";
import "dayjs/locale/en";
import relativeTime from "dayjs/plugin/relativeTime";
import duration from "dayjs/plugin/duration";
import { locale } from '@tauri-apps/plugin-os';

export async function configureDayjs() {
  dayjs.extend(relativeTime);
  dayjs.extend(duration);

  try {
    const systemLocale = await locale();
    const isJapanese = systemLocale?.toLowerCase().startsWith("ja");
    dayjs.locale(isJapanese ? "ja" : "en");
  } catch (error) {
    console.warn("Failed to get system locale, using default (ja):", error);
    dayjs.locale("ja");
  }
}
