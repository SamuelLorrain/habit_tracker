pub mod database_handling;

use regex::Regex;
use std::collections::HashMap;
use std::env;

use database_handling::*;

pub enum ArgumentTypes<A,B,C> {
    TypeA(A),
    TypeB(B),
    TypeC(C),
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
        \tx --freq <NAME> <FREQ> <FREQ_UNIT> [OPTIONS] Change frequency of the habit\n\
        \tx --time <NAME> <TIME>                       Change time of the habit\n\
        \tx --begin <NAME> <DATE>                      Change begin date of the habit (default: today)\n\
        \tx --end  <NAME> <TIME> <TIME_TYPE>           Add endtime for the habit (default: none)\n\
        \t--meta <NAME> <META>                       Add metadata to the habit\n\
        \t--missing <NAME>                           List every day the habit has been missed\n\n\
        \t--help                                     Show help\n\
        "
    );
}

pub fn handle_args() -> HashMap<&'static str, ArgumentTypes<bool, String, Vec<String>>> {
    let mut options: HashMap<&str, ArgumentTypes<bool, String, Vec<String>>> = HashMap::new();

    let args : Vec<_> =  env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if Regex::new(r"^--list$").unwrap().is_match(&args[i]) {
            options.insert("LIST", ArgumentTypes::TypeA(true));
        }
        else if Regex::new(r"^--today$").unwrap().is_match(&args[i]) {
            options.insert("TODAY", ArgumentTypes::TypeA(true));
        }
        else if Regex::new(r"^--done$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --done argument: name of the habit");
            }
            let name = &args[i+1];
            options.insert("DONE", ArgumentTypes::TypeB(name.to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--new$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --new argument: name of the habit");
            }
            options.insert("NEW", ArgumentTypes::TypeB(args[i+1].to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--freq$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --freq
            options.insert("FREQ", ArgumentTypes::TypeA(true));
            i+= 1;
        }
        else if Regex::new(r"^--time$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --time
            options.insert("TIME", ArgumentTypes::TypeA(true));
            i+= 1;
        }
        else if Regex::new(r"^--begin$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --begin argument: name of the habit");
            }
            options.insert("BEGIN", ArgumentTypes::TypeB(args[i+1].to_string()));
            i+=1
        }
        else if Regex::new(r"^--end$").unwrap().is_match(&args[i]) {
            // IMPLEMENT --end
            if i+1 < args.len() {
                panic!("Missing --end argument: name of the habit");
            }
            options.insert("END", ArgumentTypes::TypeB(args[i+1].to_string()));
        }
        else if Regex::new(r"^--meta$").unwrap().is_match(&args[i]) {
            if i+2 < args.len() {
                panic!("Missing --meta argument: name of the habit");
            }
            let meta_args = vec![args[i+1].to_string(), args[i+2].to_string()];
            options.insert("META", ArgumentTypes::TypeC(meta_args));
            i+= 2;
        }
        else if Regex::new(r"^--missing$").unwrap().is_match(&args[i]) {
            if i+1 < args.len() {
                panic!("Missing --missing argument: name of the habit");
            }
            options.insert("MISSING", ArgumentTypes::TypeB(args[i+1].to_string()));
            i+=1;
        }
        else if Regex::new(r"^--help$").unwrap().is_match(&args[i]) {
            options.insert("HELP", ArgumentTypes::TypeA(true));
        }

        i+=1;
    }

    options
}

pub fn handle_cli() {
    const FILE_NAME: &str = "habit_database.json";
    let options = handle_args();

    let mut db = open_database(FILE_NAME);

    match options.get(&"LIST") {
        Some(_) => printdb(db.as_slice()),
        _ => ()
    }
    match options.get(&"TODAY") {
        Some(_) => printdb_today(db.as_slice()),
        _ => ()
    }
    match options.get(&"DONE") {
        Some(ArgumentTypes::TypeB(name)) => {
            done_habit_in_db(&mut db, &name.to_string());
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"NEW") {
        Some(ArgumentTypes::TypeB(name)) => {
            new_habit_in_db(&mut db, &name.to_string());
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"META") {
        Some(ArgumentTypes::TypeC(data)) => {
            meta_habit_in_db(&mut db, &data[0], &data[1]);
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"MISSING") {
        Some(ArgumentTypes::TypeB(name)) => missing_habit_in_db(&mut db, &name.to_string()),
        _ => ()
    }
    match options.get(&"HELP") {
        Some(_) => {
            print_help();
            ()
        },
        _ => ()
    }
}
