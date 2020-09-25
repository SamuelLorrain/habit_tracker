use chrono::{DateTime, FixedOffset};

pub struct Habit {
    pub name: String,
    pub date_begin: Option<DateTime<FixedOffset>>,
    pub date_end: Option<DateTime<FixedOffset>>,
}

impl Habit {
    pub fn new(name: String, date_begin: Option<DateTime<FixedOffset>>, date_end: Option<DateTime<FixedOffset>>) -> Habit{
        Habit{
            name,
            date_begin,
            date_end
        }
    }

    pub fn check_end_date(&mut self, &date_end: &DateTime<FixedOffset>) -> Result<&mut Habit, &str>{
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
