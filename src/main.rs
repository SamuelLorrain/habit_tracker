pub mod habit;

use chrono::{DateTime, FixedOffset, Utc, Duration};
use habit::{Habit};

#[derive(Debug)]
struct Sprint {
    name: Option<String>,
    habits: Vec<Habit>,
    date_begin: DateTime<FixedOffset>,
    date_end: DateTime<FixedOffset>,
    metadata: Option<String>
}

fn main() {
    let date = Utc::now().naive_utc().date();
    let new_date = date.succ();
    let week = Duration::weeks(4);
    let new_date_plus_one_week = new_date + week;
    let h = Habit::default();

    println!("{}", date);
    println!("{}", new_date);
    println!("{}", new_date_plus_one_week);
    println!("{}", date.format("%Y-%m-%d %B %A"));
    println!("{:?}", h);
}
