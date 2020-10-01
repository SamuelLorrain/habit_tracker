pub mod habit;

use chrono::{DateTime, FixedOffset};
use habit::{Habit};
use std::fs;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;
use regex::Regex;
use habit::habittools::*;

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

pub enum Either<A,B> {
    Left(A),
    Right(B)
}

pub fn handle_args() -> HashMap<&'static str, Either<bool, String>> {
    let mut options: HashMap<&str, Either<bool, String>> = HashMap::new();

    let args : Vec<_> =  env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if Regex::new(r"^--list$").unwrap().is_match(&args[i]) {
            options.insert("LIST", Either::Left(true));
        }
        else if Regex::new(r"^--today$").unwrap().is_match(&args[i]) {
            options.insert("TODAY", Either::Left(true));
        }
        else if Regex::new(r"^--done$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --done argument: name of the habit");
            }
            let name = &args[i+1];
            options.insert("DONE", Either::Right(name.to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--new$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --new argument: name of the habit");
            }
            options.insert("NEW", Either::Right(args[i+1].to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--freq$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --freq
            options.insert("FREQ", Either::Left(true));
            i+= 1;
        }
        else if Regex::new(r"^--time$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --time
            options.insert("TIME", Either::Left(true));
            i+= 1;
        }
        else if Regex::new(r"^--begin$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --begin argument: name of the habit");
            }
            options.insert("BEGIN", Either::Right(args[i+1].to_string()));
        }
        else if Regex::new(r"^--end$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --end
            if i+1 < args.len() {
                panic!("Missing --end argument: name of the habit");
            }
            options.insert("END", Either::Right(args[i+1].to_string()));
        }
        else if Regex::new(r"^--meta$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --meta
            if i+1 < args.len() {
                panic!("Missing --meta argument: name of the habit");
            }
            options.insert("META", Either::Right(args[i+1].to_string()));
        }
        else if Regex::new(r"^--missing$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --missing
            if i+1 < args.len() {
                panic!("Missing --missing argument: name of the habit");
            }
            options.insert("MISSING", Either::Right(args[i+1].to_string()));
        }
        else if Regex::new(r"^--help$").unwrap().is_match(&args[i]) {
            options.insert("HELP", Either::Left(true));
            i+= 1;
        }

        i+=1;
    }

    options
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

pub fn new_habit_in_db(db: &mut Vec<Habit>, newHabit: &str) {
    let mut h = Habit::default();
    h.set_name(newHabit);
    db.push(h);
}

pub fn print_help() {
    print!(
       "habit_tracker\n\
        A simple habit tracker with stats\n\n\
        USAGE:\n\
        \thabit_tracker [OPTIONS]\n\n\
        OPTIONS:\n\
        \t--list                                     List all current habits\n\
        \t--today                                    List all habits todo today\n\
        \t--done <NAME>                              Mark a habit has done (if it is due today)\n\
        \t--new <NAME>                               Create a new habit\n\
        \t--freq <NAME> <FREQ> <FREQ_UNIT> [OPTIONS] Change frequency of the habit\n\
        \t--time <NAME> <TIME>                       Change time of the habit\n\
        \t--begin <NAME> <DATE>                      Change begin date of the habit (default: today)\n\
        \t--end  <NAME> <TIME> <TIME_TYPE>           Add endtime for the habit (default: none)\n\
        \t--meta <NAME> <META>                       Add metadata to the habit\n\
        \t--missing <NAME>                           List every day the habit has been missed\n\n\
        \t--help                                     Show help\n\
        "
    );
}

fn main() {
    const FILE_NAME: &str = "habit_database.json";
    let options = handle_args();

    let mut db = open_database(FILE_NAME);

    //let h = Habit::default();
    //db.push(h);
    //println!("{:?}", db);


    match options.get(&"HELP") {
        Some(_) => {
            print_help();
            ()
        },
        _ => ()
    }
    match options.get(&"LIST") {
        Some(_) => printdb(db.as_slice()),
        _ => ()
    }
    match options.get(&"TODAY") {
        Some(_) => printdb_today(db.as_slice()),
        _ => ()
    }
    match options.get(&"NEW") {
        Some(Either::Right(name)) => new_habit_in_db(&mut db, &name.to_string()),
        _ => ()
    }

    //save_database(db, FILE_NAME)
    //    .expect("Unable to save database");

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

}
