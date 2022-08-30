import { dateToJapaneseFormatString } from "./calc";

describe("dateToJapaneseFormatString", () => {
  it("format のテスト", () => {
    const time = new Date(2022, 10, 10);
    const actual = dateToJapaneseFormatString(time);

    expect(actual).toBe("令和4年11月10日");
  });
});
