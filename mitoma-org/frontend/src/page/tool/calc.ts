import { intervalToDuration } from "date-fns";

export function dateToJapaneseFormatString(date: Date): string {
  const eraFormat = new Intl.DateTimeFormat("ja-JP-u-ca-japanese", {
    era: "short",
    year: "numeric",
    month: "long",
    day: "numeric",
  });
  return eraFormat.format(date);
}

export function dateToString(date: Date): string {
  const adFormat = new Intl.DateTimeFormat("ja-JP", {
    era: "short",
    year: "numeric",
    month: "long",
    day: "numeric",
  });
  return adFormat.format(date);
}

export function intervalString(start: Date, end: Date): string {
  const duration = intervalToDuration({ start, end });
  return `${duration.years} 年 ${duration.months} カ月 ${duration.days} 日`;
}
