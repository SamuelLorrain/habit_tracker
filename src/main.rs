
pub mod habit;

use chrono::{DateTime, FixedOffset};

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
    let date_begin = DateTime::parse_from_rfc3339("2020-09-19T16:39:57-08:00")
        .expect("Test");
    let date_end = DateTime::parse_from_rfc3339("2020-10-19T16:39:57-08:00")
        .expect("Test");

    let mut h = Habit::default();

    //let mut s = Sprint::new(
    //    None,
    //    date_begin,
    //    date_end,
    //    Some(String::from("Test"))
    //);
    //s.add_habit(h);

    println!("{:?}", h);
}
