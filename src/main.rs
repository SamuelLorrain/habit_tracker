
mod habit;

use chrono::{DateTime, FixedOffset};
use habit::{Habit};

fn main() {
    let date_begin = DateTime::parse_from_rfc3339("2020-09-19T16:39:57-08:00")
        .expect("Test");
    let date_end = DateTime::parse_from_rfc3339("2020-10-19T16:39:57-08:00")
        .expect("Test");

    let new_date_end = DateTime::parse_from_rfc3339("2020-11-19T16:39:57-08:00")
        .expect("Test");

    let mut h = Habit::new(
        String::from("Test"),
        Some(date_begin),
        Some(date_end),
        None,
        None,
    );

    match h.check_end_date(&new_date_end) {
        Ok(_) => println!("Date successfully changed"),
        Err(x) => eprintln!("{}", x)
    }

    println!("{}", h);
}
