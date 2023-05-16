/**
 * Unix 时间戳，带符号整数
 * - `1970/01/01 00:00:00` 为 0
 */
type NumTimestamp = i32;

const MS_PER_DAY: f64 = 86400000.0; // 24*60*60*1000

/**
 * 根据 Excel 的定义：
 * - 整数部分：与 1900/01/01 相差的天数 (1900/01/01 这天的 OADate 值为 1)；
 * - 小数部分：时分秒；
 *
 * 但 Excel 错误地将 1900 年作为闰年：
 * - 各相关产品为尽可能与 Excel 兼容，将基准日期改为 `1899/12/30`。这也使在表示 `1900/03/01` 之前的日期时，会与 Excel 的值相差 1；
 *     - `1899/12/30 00:00:00` 为 0.0;
 * - Excel 为弥补该差异，之后添加了以 `1904/01/01` 为基准的选项。
 *     - `1904/01/01 00:00:00` 为 0.0;
 */
type NumOADate = f64;

/** days between 1899/12/30 and 1970/01/01 (Unix Timestamp) */
const OFFSET_1900: f64 = 25569.0;

/** days between 1904/01/01 and 1970/01/01 */
const OFFSET_1904: f64 = OFFSET_1900 - 1462.0; // 4 个平年 + 2 天 (1899/12/30, 12/31) => 1462

// LATER: Excel 对 OADate 有最大值最小值显示，我们暂时先不考虑
// const MAX_VALUE_1900 = 2958466.9999999884;
// const MAX_VALUE_1904 = 2957003.9999999884;

pub fn from_timestamp(timestamp: NumTimestamp, is_1904: bool) -> NumOADate {
  let offset = if is_1904 { OFFSET_1904 } else { OFFSET_1900 };
  (timestamp as NumOADate / MS_PER_DAY) - offset
}

pub fn to_timestamp(oadate: NumOADate, is_1904: bool) -> NumTimestamp {
  let offset = if is_1904 { OFFSET_1904 } else { OFFSET_1900 };
  let result_day = (oadate - offset) * MS_PER_DAY;
  result_day as NumTimestamp
}

#[cfg(test)]
mod test {
  use chrono::{ NaiveDate };
  use crate::{ NumOADate, NumTimestamp, from_timestamp, to_timestamp };

  type TestCase = (NumTimestamp, NumOADate/* 1900 */, NumOADate/* 1904 */);

  fn from_ymd_hms(y: u32, m: u32, d: u32, h: u32, min: u32, s: u32) -> NumTimestamp {
    NaiveDate::from_ymd(y as i32, m, d).and_hms(h, min, s).timestamp() as NumTimestamp
  }

  #[test]
  fn from_timestamp_test() {
    let cases: Vec<TestCase> = vec![
      (from_ymd_hms(1850, 07, 01, 0, 0, 0), -18079.0, -19541.0),
      (from_ymd_hms(1900, 01, 01, 0, 0, 0),      2.0,  -1460.0),
      (from_ymd_hms(1900, 03, 01, 0, 0, 0),     61.0,  -1401.0),
      (from_ymd_hms(1904, 01, 01, 0, 0, 0),   1462.0,      0.0),
      (from_ymd_hms(1904, 03, 01, 0, 0, 0),   1522.0,     60.0),
      (from_ymd_hms(1970, 01, 01, 0, 0, 0),  25569.0,  24107.0),

      (from_ymd_hms(1904, 01, 01, 23, 10, 05),  1046.96533564815, -415.03466435185),
      (from_ymd_hms(2020, 01, 01, 12, 45, 0),  43831.53125,      42369.53125),
      (from_ymd_hms(2020, 01, 01, 12, 45, 23), 43831.5315162037, 42369.5315162037),
    ];

    for (datetime, date1900, date1904) in cases {
      assert_eq!(from_timestamp(datetime, false), date1900);
      assert_eq!(from_timestamp(datetime, true), date1904);

      assert_eq!(to_timestamp(date1900, false), datetime);
      assert_eq!(to_timestamp(date1904, false), datetime);
    }
  }
}
