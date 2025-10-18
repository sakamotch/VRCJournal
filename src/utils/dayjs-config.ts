import "dayjs/locale/ja";
import "dayjs/locale/en";

import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
import localizedFormat from "dayjs/plugin/localizedFormat";
import relativeTime from "dayjs/plugin/relativeTime";

import type { Locale } from "@/types";

export function configureDayjs() {
  dayjs.extend(relativeTime);
  dayjs.extend(duration);
  dayjs.extend(localizedFormat);
}

export function setDayjsLocale(locale: Locale) {
  dayjs.locale(locale);
}
