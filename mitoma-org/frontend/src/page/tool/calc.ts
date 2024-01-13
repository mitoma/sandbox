import { format, intervalToDuration, parse } from "date-fns";

export function immutableSwap<T>(
  items: Array<T>,
  firstIndex: number,
  secondIndex: number
): Array<T> {
  if (
    firstIndex < 0 ||
    firstIndex >= items.length ||
    secondIndex < 0 ||
    secondIndex >= items.length
  ) {
    return items;
  }

  const results = items.slice();
  const firstItem = items[firstIndex];
  results[firstIndex] = items[secondIndex];
  results[secondIndex] = firstItem;
  return results;
}

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
  const years = duration.years ?? 0;
  const months = duration.months ?? 0;
  const days = duration.days ?? 0;
  return `${years} 年 ${months} カ月 ${days} 日`;
}

export function parseForLocalStorage(dateStr: string): Date {
  return parse(dateStr, "yyyy-MM-dd", new Date(1980, 1, 1, 0, 0, 0, 0));
}

export function formatForLocalStorage(date: Date): string {
  return format(date, "yyyy-MM-dd");
}
