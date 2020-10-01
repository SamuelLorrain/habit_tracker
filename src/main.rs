pub mod habit;

use chrono::{DateTime, FixedOffset, Utc, Weekday};
use habit::{Habit};

use habit::habittools::*;

use std::fs;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct Sprint {
    name: Option<String>,
    habits: Vec<Habit>,
    date_begin: DateTime<FixedOffset>,
    date_end: DateTime<FixedOffset>,
    metadata: Option<String>
}

fn main() {
    const FILE_NAME: &str = "habit_database.json";
    //let mut h = Habit::default();

    //h.set_name("Do sports");
    //h.set_metadata(&Some(String::from("Test")));
    //h.set_metadata(&None);
    //h.set_date_begin(&Utc::now().naive_utc().date().pred().pred().pred());
    //h.set_time_habit(&Some(Utc::now().naive_utc().time()));
    //h.set_end_type(&EndRepeatType::AfterOccurrences(5));

    //week
    //h.set_time_unit(&RepeatTimeUnit::Weeks);
    //h.set_weekdays(&Some(vec![Weekday::Tue, Weekday::Fri]));

    //month
    //h.set_time_unit(&RepeatTimeUnit::Months);
    //h.set_repeat_month(&Some(RepeatMonth::DayOfMonth(29)));
    //h.set_repeat_month(&Some(RepeatMonth::DayOfWeek(2, Weekday::Thu)));

    //day
    //h.set_time_unit(&RepeatTimeUnit::Days);
    //h.set_time_repeat(1);
    //h.show();

    //h.push_history(&Utc::now().naive_utc(), &None);
    //println!("{:?}", h.history().last());

    //println!("{:?}", h.todo_today());

    //println!("{:?}", h.todo_today());
    ////h.done(None).expect("Error can't be done");
    //println!("{:?}", h.history());

    //let h_iter = h.limit_date_iter();

    //for x in h_iter {
    //    println!("{:?}", x);
    //}

    //println!("{:?}", h.next_time());

    //let serialized = serde_json::to_string(&h).unwrap();

    //let mut file = fs::File::create("test_file.json")
    //    .expect("NEE");
    //let serialized = serialized.as_bytes();
    //file.write_all(serialized)
    //    .expect("Unable to write");

    //println!("{:?}", serialized);
    let file = fs::File::open(FILE_NAME)
        .expect("Unable to read file");
    let reader = BufReader::new(file);
    let mut de = serde_json::Deserializer::from_reader(reader);
    let h = Habit::deserialize(&mut de)
        .expect("The json file is not a valid habit database");

    h.show();
}
