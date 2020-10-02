use crate::habit::habittools::*;
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
        }
        println!("============");
    }
}

pub fn new_habit_in_db(db: &mut Vec<Habit>, new_habit: &str) {
    let mut h = Habit::default();
    h.set_name(new_habit);
    db.push(h);
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

pub fn missing_habit_in_db(db: &mut Vec<Habit>, habit_name: &str) {
    for x in db {
        if x.name() == habit_name {
            let iter = x.has_missing_iter();
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

