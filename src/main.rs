pub mod habit;

use chrono::{DateTime, FixedOffset, Utc, Weekday};
use habit::{Habit};

use habit::habittools::*;

#[derive(Debug)]
struct Sprint {
    name: Option<String>,
    habits: Vec<Habit>,
    date_begin: DateTime<FixedOffset>,
    date_end: DateTime<FixedOffset>,
    metadata: Option<String>
}

fn main() {
    let mut h = Habit::default();

    h.set_name("Do sports");
    h.set_metadata(&Some(String::from("Test")));
    h.set_metadata(&None);
    h.set_date_begin(&Utc::now().naive_utc().date());
    h.set_time_habit(&Some(Utc::now().naive_utc().time()));
    h.set_end_type(&EndRepeatType::AfterOccurrences(5));

    //week
    //h.set_time_unit(&RepeatTimeUnit::Weeks);
    //h.set_weekdays(&Some(vec![Weekday::Tue, Weekday::Fri]));

    //month
    h.set_time_unit(&RepeatTimeUnit::Months);
    h.set_repeat_month(&Some(RepeatMonth::DayOfMonth(29)));
    //h.set_repeat_month(&Some(RepeatMonth::DayOfWeek(2, Weekday::Thu)));

    //day
    //h.set_time_unit(&RepeatTimeUnit::Days);
    //h.set_time_repeat(2);
    h.show();


    //h.push_history(&Utc::now().naive_utc(), &None);
    //println!("{:?}", h.history().last());

    //println!("{:?}", h.todo_today());

    println!("{:?}", h.todo_today());
}
