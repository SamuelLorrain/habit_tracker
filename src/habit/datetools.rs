use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Weekday, Utc, Duration};

pub fn getCurrentWeekday() -> Weekday {
    Weekday::from_usize(
        Utc::now()
        .naive_utc()
        .format("%u")
        .to_string()
        .parse::<usize>()
        .unwrap()
        - 1 as usize
    ).unwrap();
}
