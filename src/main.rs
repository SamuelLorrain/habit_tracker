pub mod habit;

use chrono::{DateTime, FixedOffset};
use habit::{Habit};
use std::fs;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

#[derive(Debug)]
struct Sprint {
    name: Option<String>,
    habits: Vec<Habit>,
    date_begin: DateTime<FixedOffset>,
    date_end: DateTime<FixedOffset>,
    metadata: Option<String>
}

pub fn open_database(database_path: &str) -> Vec<Habit> {
    let file = match fs::File::open(database_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}, creating new database", err);
            let mut new_db = fs::File::create(database_path)
                .expect("Unable to create database");
            let db_vector : Vec<Habit> = Vec::new();
            let serialized = serde_json::to_string(&db_vector)
                .expect("Unable to create database (serde_json error)");
            new_db.write_all(serialized.as_bytes())
                .expect("Unable to write in the new created file");
            new_db
        }
    };

    let reader = BufReader::new(file);

    let array: Vec<Habit> = serde_json::from_reader(reader)
        .expect("error from reader");

    array
}

pub fn save_database(database: Vec<Habit>, database_path: &str) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(database_path)
        .expect("File doesn't exist");
    let serialized = serde_json::to_string(&database)
        .expect("Unable to store create database, not a valid database");
    file.write_all(serialized.as_bytes())
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
    //println!("{:?}", h.history());
    //println!("{:?}", h.next_time());

    let mut db = open_database(FILE_NAME);

    let h = Habit::default();
    db.push(h);
    println!("{:?}", db);

    save_database(db, FILE_NAME)
        .expect("Unable to save database");
}
