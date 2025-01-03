use chrono::{NaiveDate, Datelike, Weekday};

fn from_ymd(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn main() {
    let mut count = 0;

    for year in 1901..=2000 {
        for month in 1..=12 {
            let date = from_ymd(year as i32, month as u32, 1);
            if date.weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }
    
    println!("count={count}");

    // println!("{:?}", date.weekday());
}
