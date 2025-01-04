use chrono::{Datelike, NaiveDate, Weekday};

pub fn problem19() -> i64 {
    let mut count = 0;

    for year in 1901i32..=2000i32 {
        for month in 1u32..=12u32 {
            let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            if date.weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }

    count
}
