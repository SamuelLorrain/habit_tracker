use crate::habit::habittools::*;
use crate::habit::datetools::{parse_weekday};
use crate::habit::{Habit};
use std::io::BufReader;
use std::io::Error;
use std::fs;
use std::io::prelude::*;

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

pub fn save_database(database: &Vec<Habit>, database_path: &str) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(database_path)
        .expect("File doesn't exist");
    let serialized = serde_json::to_string(&database)
        .expect("Unable to store create database, not a valid database");
    file.write_all(serialized.as_bytes())
}

pub fn printdb(db: &[Habit]) {
    println!("============");
    for x in db.iter() {
        x.show();
        println!("============");
    }
}

pub fn printdb_today(db: &[Habit]) {
    println!("Today, you must do :");
    println!("============");
    for x in db.iter() {
        if x.todo_today() == HabitInfo::TodoToday {
            x.show();
            println!("============");
        }
    }
}

pub fn new_habit_in_db(db: &mut Vec<Habit>, new_habit: &str) {
    match db.iter().position(|x| x.name() == new_habit) {
        Some(x) => panic!("Unable to create two habit with the same name"),
        None => {
            let mut h = Habit::default();
            h.set_name(new_habit);
            db.push(h);
        }
    }
}

pub fn done_habit_in_db(db: &mut Vec<Habit>, habit_name: &str) {
    let mut is_done = false;
    for x in db {
        if x.name() == habit_name {
            is_done = true;
            x.done(None)
                .expect("Error the habit has not to be done");
        }
    }
    if ! is_done {
        panic!("the habit doesn't exist");
    }
}

pub fn missing_habit_in_db(db: &Vec<Habit>, habit_name: &str) {
    for x in db {
        if x.name() == habit_name {
            let iter = x.has_missing_iter();
            println!("You have missed the following days for `{}` :", habit_name);
            for y in iter {
                println!("{}", y);
            }
        }
    }
}

pub fn meta_habit_in_db(db: &mut Vec<Habit>, habit_name: &str, meta: &str) {
    for x in db {
        if x.name() == habit_name {
            x.set_metadata(&Some(meta.to_string()));
        }
    }
}

pub fn time_habit_in_db(db: &mut Vec<Habit>, habit_name: &str, time: &str) {
    for x in db {
        if x.name() == habit_name {
            let time_parsed = chrono::NaiveTime::parse_from_str(time, "%H:%M:%S")
                .expect("Unable to parse time");
            x.set_time_habit(&Some(time_parsed));
        }
    }
}

pub fn begin_habit_in_db(db: &mut Vec<Habit>, habit_name: &str, date_begin: &str) {
    for x in db {
        if x.name() == habit_name {
            let date_begin_parsed = chrono::NaiveDate::parse_from_str(date_begin, "%Y-%m-%d")
                .expect("Unable to parse date");
            x.set_date_begin(&date_begin_parsed);
        }
    }
}

pub fn end_habit_in_db(db: &mut Vec<Habit>,
                       habit_name: &str,
                       time_or_occurrence: &str,
                       end_type: &str) {

    if end_type.to_lowercase() == "never" {
        for x in db {
            if x.name() == habit_name {
                x.set_end_type(&EndRepeatType::Never);
            }
        }
    }
    else if end_type.to_lowercase() == "after_occurrences" {
        let time : usize = time_or_occurrence.parse()
            .expect("Error time could not be parsed");
        for x in db {
            if x.name() == habit_name {
                x.set_end_type(&EndRepeatType::AfterOccurrences(time));
            }
        }
    }
    else if end_type.to_lowercase() == "on" {
        let date = chrono::NaiveDate::parse_from_str(time_or_occurrence, "%Y-%m-%d")
        .expect("Error time could not be parsed");
        for x in db {
            if x.name() == habit_name {
                x.set_end_type(&EndRepeatType::On(date));
            }
        }
    }
}

pub fn freq_habit_in_db(db: &mut Vec<Habit>,
                       habit_name: &str,
                       frequency: &str,
                       frequency_unit: &str,
                       options: Option<Vec<String>>) {
    for x in db {
        if x.name() == habit_name {
            let time_repeat : usize = frequency.parse()
                .expect("Error the frequency provided is not a number");
            x.set_time_repeat(time_repeat);
            let time_unit = match frequency_unit {
                "days"   => RepeatTimeUnit::Days,
                "weeks"  => RepeatTimeUnit::Weeks,
                "months" => RepeatTimeUnit::Months,
                "years"  => RepeatTimeUnit::Years,
                _        => panic!("Bad frequency unit entered")
            };
            x.set_time_unit(&time_unit);
            match &options {
                Some(weeks_str_vec) if time_unit == RepeatTimeUnit::Weeks => {
                    let weeks = weeks_str_vec.iter().map(|x| parse_weekday(x).unwrap()).collect();
                    x.set_weekdays(&Some(weeks));
                },
                Some(months_str_vec) if time_unit == RepeatTimeUnit::Months => {
                    let month_type = &months_str_vec[0];
                    match month_type.as_ref() {
                        "day_of_month" => {
                            let day_of_month: usize = months_str_vec[1].parse().unwrap();
                            x.set_repeat_month(
                                &Some(RepeatMonth::DayOfMonth(day_of_month)));
                        },
                        "day_of_week" => {
                            let day_of_week: usize = months_str_vec[1].parse().unwrap();
                            x.set_repeat_month(
                                &Some(RepeatMonth::DayOfWeek(
                                        day_of_week, parse_weekday(&months_str_vec[2]).unwrap())));
                        }
                        _ => panic!("Bad repeat frequency type entered")
                    }
                }
                _ => (),
            }
        }
    }
}

pub fn history_habit_in_db(db: &Vec<Habit>, habit_name: &str) {
    for x in db {
        if x.name() == habit_name {
            for y in x.history().iter() {
                y.show();
            }
        }
    }
}

pub fn delete_habit_from_db(db: &mut Vec<Habit>, habit_name: &str) {
    if let Some(pos) = db.iter().position(|x| x.name() == habit_name) {
        db.remove(pos);
    }
}
