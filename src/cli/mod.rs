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
        \t--freq <NAME> <FREQ> <FREQ_UNIT> [OPTIONS] Change frequency of the habit\n\
        \t--time <NAME> <TIME>                       Change time of the habit\n\
        \t--begin <NAME> <DATE>                      Change begin date of the habit (default: today)\n\
        \t--end  <NAME> <TIME> <TIME_TYPE>           Add endtime for the habit (default: none)\n\
        \t--meta <NAME> <META>                       Add metadata to the habit\n\
        \t--history <NAME>                           History for the given habits\n\
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
        // use macro
        if Regex::new(r"^--list$").unwrap().is_match(&args[i]) {
            options.insert("LIST", ArgumentTypes::TypeA(true));
        }
        else if Regex::new(r"^--today$").unwrap().is_match(&args[i]) {
            options.insert("TODAY", ArgumentTypes::TypeA(true));
        }
        else if Regex::new(r"^--done$").unwrap().is_match(&args[i]) {
            if i+1 >= args.len() {
                panic!("Missing --done argument: name of the habit");
            }
            let name = &args[i+1];
            options.insert("DONE", ArgumentTypes::TypeB(name.to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--new$").unwrap().is_match(&args[i]) {
            if i+1 >= args.len() {
                panic!("Missing --new argument: name of the habit");
            }
            options.insert("NEW", ArgumentTypes::TypeB(args[i+1].to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--freq$").unwrap().is_match(&args[i]) {
            if i+3 >= args.len() {
                panic!("Missing --freq argument");
            }
            let mut infos = vec![];
            for x in args[i+1..].iter() {
                infos.push(x.clone());
            }
            options.insert("FREQ", ArgumentTypes::TypeC(infos));
            i += args.len();
        }
        else if Regex::new(r"^--time$").unwrap().is_match(&args[i]) {
            if i+2 >= args.len() {
                panic!("Missing --time argument: <name> <time>");
            }
            let time_args = vec![args[i+1].to_string(), args[i+2].to_string()];
            options.insert("TIME", ArgumentTypes::TypeC(time_args));
            i+= 2;
        }
        else if Regex::new(r"^--begin$").unwrap().is_match(&args[i]) {
            if i+2 >= args.len() {
                panic!("Missing --begin argument: name of the habit");
            }
            let begin_args = vec![args[i+1].to_string(), args[i+2].to_string()];
            options.insert("BEGIN", ArgumentTypes::TypeC(begin_args));
            i+= 2;
        }
        else if Regex::new(r"^--end$").unwrap().is_match(&args[i]) {
            if i+3 >= args.len() {
                panic!("Missing --end argument: <name> <time> <time_type>");
            }
            let end_args = vec![args[i+1].to_string(),
                                args[i+2].to_string(),
                                args[i+3].to_string()];
            options.insert("END", ArgumentTypes::TypeC(end_args));
            i+=3;
        }
        else if Regex::new(r"^--meta$").unwrap().is_match(&args[i]) {
            if i+2 >= args.len() {
                panic!("Missing --meta argument: name of the habit");
            }
            let meta_args = vec![args[i+1].to_string(), args[i+2].to_string()];
            options.insert("META", ArgumentTypes::TypeC(meta_args));
            i+= 2;
        }
        else if Regex::new(r"^--history$").unwrap().is_match(&args[i]) {
            if i+1 >= args.len() {
                panic!("Missing --history argument: name of the habit");
            }
            options.insert("HISTORY", ArgumentTypes::TypeB(args[i+1].to_string()));
            i+= 1;
        }
        else if Regex::new(r"^--missing$").unwrap().is_match(&args[i]) {
            if i+1 >= args.len() {
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

    if options.len() == 0 {
        print_help();
        return;
    }

    let mut db = open_database(FILE_NAME);

    //use macro
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
    match options.get(&"FREQ") {
        Some(ArgumentTypes::TypeC(data)) => {
            let mut vector_options: Vec<String> = vec![];
            if data.len() > 2 {
                for x in data[3..].iter() {
                    vector_options.push(x.to_string());
                }
            }
            if vector_options.len() == 0 {
                freq_habit_in_db(
                    &mut db,
                    &data[0],
                    &data[1],
                    &data[2],
                    Some(vector_options)
                );
            } else {
                freq_habit_in_db(
                    &mut db,
                    &data[0],
                    &data[1],
                    &data[2],
                    None
                );
            }
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"TIME") {
        Some(ArgumentTypes::TypeC(data)) => {
            time_habit_in_db(&mut db, &data[0], &data[1]);
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"BEGIN") {
        Some(ArgumentTypes::TypeC(data)) => {
            begin_habit_in_db(&mut db, &data[0], &data[1]);
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"END") {
        Some(ArgumentTypes::TypeC(data)) => {
            end_habit_in_db(&mut db, &data[0], &data[1], &data[2]);
            save_database(&db, FILE_NAME)
                .expect("Unable to save database");
        },
        _ => ()
    }
    match options.get(&"HISTORY") {
        Some(ArgumentTypes::TypeB(data)) => {
            history_habit_in_db(&mut db, &data);
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
