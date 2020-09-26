use chrono::{DateTime, FixedOffset, Duration};
use std::fmt;

#[derive(Debug)]
pub struct Habit {
    name: String,
    date_begin: Option<DateTime<FixedOffset>>,
    date_end: Option<DateTime<FixedOffset>>,
    interval: Option<Duration>,
    done: bool,
    metadata: Option<String>
}

impl Habit {
    pub fn new(name: String,
               date_begin: Option<DateTime<FixedOffset>>,
               date_end: Option<DateTime<FixedOffset>>,
               interval: Option<Duration>,
               metadata: Option<String>) -> Habit{
        Habit{
            name,
            date_begin,
            date_end,
            interval,
            done: false,
            metadata
        }
    }

    pub fn check_end_date(&mut self, &date_end: &DateTime<FixedOffset>) -> Result<&mut Habit, &str> {
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

    pub fn check_begin_date(&mut self, &date_begin: &DateTime<FixedOffset>) -> Result<&mut Habit, &str> {
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

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date_begin = match self.date_begin {
            Some(x) => x.to_string(),
            None => String::from("")
        };

        let date_end = match self.date_end {
            Some(x) => x.to_string(),
            None => String::from("")
        };

        let interval = ""; //No proper implementation yet

        let done = match self.done {
            false => "It has not been done.",
            true  => "It has been done.",
        };

        let metadata = match &self.metadata {
            Some(x) => x.to_string(),
            None => String::from("")
        };

        write!(f, "===========\n\
                  name: {}\n\
                  begin: {}\n\
                  end: {}\n\
                  interval: {}\n\
                  {}\n\
                  metadata: {}\n\
                  ===========",
                  self.name, date_begin, date_end, interval, done, metadata)
    }
}
