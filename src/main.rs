
mod habit;

use chrono::{DateTime, FixedOffset};

use habit::{Habit};
use std::ptr;

#[derive(Debug)]
struct Sprint {
    name: Option<String>,
    habits: Vec<Habit>,
    date_begin: DateTime<FixedOffset>,
    date_end: DateTime<FixedOffset>,
    metadata: Option<String>
}

impl Sprint {
    fn new(name: Option<String>,
           date_begin: DateTime<FixedOffset>,
           date_end: DateTime<FixedOffset>,
           metadata: Option<String>) -> Sprint {
        Sprint {
            name,
            habits: Vec::new(),
            date_begin,
            date_end,
            metadata
        }
    }

    fn add_habit(&mut self, h: Habit){
        self.habits.push(h);
    }

    fn delete_habit(&mut self, h: &Habit){
        let index = self.habits.iter().position(|x| ptr::eq(x, h))
            .expect("Element invalid");
        self.habits.remove(index);
    }
}

//impl Display for Sprint {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//    }
//}


fn main() {
    let date_begin = DateTime::parse_from_rfc3339("2020-09-19T16:39:57-08:00")
        .expect("Test");
    let date_end = DateTime::parse_from_rfc3339("2020-10-19T16:39:57-08:00")
        .expect("Test");

    let new_date_end = DateTime::parse_from_rfc3339("2020-11-19T16:39:57-08:00")
        .expect("Test");

    let new_date_begin = DateTime::parse_from_rfc3339("2020-08-19T16:39:57-08:00")
        .expect("Test");

    let mut h = Habit::new(
        String::from("Test"),
        Some(date_begin),
        Some(date_end),
        None,
        None,
    );

    match h.check_end_date(&new_date_end) {
        Ok(_) => println!("Date successfully changed"),
        Err(x) => eprintln!("{}", x)
    }

    match h.check_begin_date(&new_date_begin) {
        Ok(_) => println!("Date successfully changed"),
        Err(x) => eprintln!("{}", x)
    }

    let mut s = Sprint::new(
        None,
        date_begin,
        date_end,
        Some(String::from("Test"))
    );

    s.add_habit(h);

    println!("{:?}", s);
}
