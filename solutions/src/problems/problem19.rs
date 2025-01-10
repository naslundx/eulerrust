use chrono::{Datelike, NaiveDate, Weekday};

pub fn problem19() -> i64 {
    let mut count = 0;

    for year in 1901..=2000 {
        for month in 1..=12 {
            let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            if date.weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }

    count
}
