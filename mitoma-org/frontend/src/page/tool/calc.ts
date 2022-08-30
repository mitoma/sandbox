export function dateToJapaneseFormatString(date: Date): string {
  const eraFormat = new Intl.DateTimeFormat("ja-JP-u-ca-japanese", {
    era: "short",
    year: "numeric",
    month: "long",
    day: "2-digit",
  });
  return eraFormat.format(date);
}

export function dateString(date: Date): string {
  const adFormat = new Intl.DateTimeFormat("ja-JP", {
    era: "short",
    year: "numeric",
    month: "long",
    day: "2-digit",
  });
  return adFormat.format(date);
}

export function pastYearDays(targetDate: Date): string {
  const currentTime: Date = new Date();
  const interval: number = targetDate.getTime() - currentTime.getTime();
  return `${interval} msec?`;
}
