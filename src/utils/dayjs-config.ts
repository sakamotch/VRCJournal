import dayjs from "dayjs";
import "dayjs/locale/ja";
import "dayjs/locale/en";
import relativeTime from "dayjs/plugin/relativeTime";
import duration from "dayjs/plugin/duration";
import localizedFormat from "dayjs/plugin/localizedFormat";
import updateLocale from "dayjs/plugin/updateLocale";
import type { Locale } from "@/i18n";

export function configureDayjs() {
  dayjs.extend(relativeTime);
  dayjs.extend(duration);
  dayjs.extend(localizedFormat);
}

export function setDayjsLocale(locale: Locale) {
  dayjs.locale(locale);
}
