

use chrono::{DateTime, FixedOffset};

struct Habit {
    name: String,
    date_begin: Option<DateTime<FixedOffset>>,
    date_end: Option<DateTime<FixedOffset>>,
}

impl Habit {
    fn new(name: String, date_begin: Option<DateTime<FixedOffset>>, date_end: Option<DateTime<FixedOffset>>) -> Habit{
        Habit{
            name,
            date_begin,
            date_end
        }
    }

    fn check_end_date(&mut self, &date_end: &DateTime<FixedOffset>) -> Result<&mut Habit, &str>{
        match self.date_begin {
            None => Ok(self),
            Some(beg) => {
                if(beg.timestamp() - date_end.timestamp()) < 0 { //diff
                    self.date_end = Some(date_end.clone());
                    return Ok(self);
                }
                Err("The date is bad")
            }
        }
    }

    fn check_begin_date(&mut self, &date_begin: &DateTime<FixedOffset>) -> Result<&mut Habit, &str>{
        match self.date_end {
            None => Ok(self),
            Some(end) => {
                if(end.timestamp() - date_begin.timestamp()) < 0 { //diff
                    self.date_begin = Some(date_begin.clone());
                    return Ok(self);
                }
                Err("The date is bad")
            }
        }
    }
}

fn main() {
    let date_begin = DateTime::parse_from_rfc3339("2020-09-19T16:39:57-08:00")
        .expect("Test");
    let date_end = DateTime::parse_from_rfc3339("2020-10-19T16:39:57-08:00")
        .expect("Test");

    let new_date_end = DateTime::parse_from_rfc3339("2020-11-19T16:39:57-08:00")
        .expect("Test");

    let mut h = Habit::new(
        String::from("Test"),
        Some(date_begin),
        Some(date_end)
    );
    println!("{} {:?} {:?}", h.name, h.date_begin, h.date_end);

    match h.check_end_date(&new_date_end) {
        Ok(_) => println!("Date successfully changed"),
        Err(x) => eprintln!("{}", x)
    }

    println!("{} {:?} {:?}", h.name, h.date_begin, h.date_end);
}

