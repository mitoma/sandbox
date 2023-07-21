import {
  dateToString,
  dateToJapaneseFormatString,
  intervalString,
  immutableSwap,
} from "./calc";

describe("immutableSwap", () => {
  it("入れかわるケース", () => {
    const actual = immutableSwap([1, 2, 3, 4], 1, 2);

    console.log(actual);
    expect(actual).toStrictEqual([1, 3, 2, 4]);
  });

  it("first が範囲外", () => {
    const actual = immutableSwap([1, 2, 3, 4], -1, 2);

    console.log(actual);
    expect(actual).toStrictEqual([1, 2, 3, 4]);
  });

  it("second が範囲外", () => {
    const actual = immutableSwap([1, 2, 3, 4], 1, 4);

    console.log(actual);
    expect(actual).toStrictEqual([1, 2, 3, 4]);
  });
});

describe("dateToJapaneseFormatString", () => {
  it("format のテスト(令和)", () => {
    const time = new Date(2022, 10, 10);
    const actual = dateToJapaneseFormatString(time);

    expect(actual).toBe("令和4年11月10日");
  });

  it("format のテスト(令和元年)", () => {
    const time = new Date(2019, 10, 1);
    const actual = dateToJapaneseFormatString(time);

    expect(actual).toBe("令和元年11月1日");
  });

  it("format のテスト(昭和)", () => {
    const time = new Date(1980, 4, 26);
    const actual = dateToJapaneseFormatString(time);

    expect(actual).toBe("昭和55年5月26日");
  });
});

describe("dateToString", () => {
  it("format のテスト(令和)", () => {
    const time = new Date(2022, 10, 10);
    const actual = dateToString(time);

    expect(actual).toBe("西暦2022年11月10日");
  });

  it("format のテスト(令和元年)", () => {
    const time = new Date(2019, 10, 1);
    const actual = dateToString(time);

    expect(actual).toBe("西暦2019年11月1日");
  });

  it("format のテスト(昭和)", () => {
    const time = new Date(1980, 4, 26);
    const actual = dateToString(time);

    expect(actual).toBe("西暦1980年5月26日");
  });
});

describe("intervalString", () => {
  it("年月日がある場合", () => {
    const from = new Date(1980, 4, 26);
    const to = new Date(2022, 5, 28);
    const actual = intervalString(from, to);

    expect(actual).toBe("42 年 1 カ月 2 日");
  });

  it("日のみ", () => {
    const from = new Date(1980, 4, 1);
    const to = new Date(1980, 4, 20);
    const actual = intervalString(from, to);

    expect(actual).toBe("0 年 0 カ月 19 日");
  });
});
