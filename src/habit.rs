use chrono::{NaiveDateTime, Duration, Utc};

#[derive(Debug)]
pub struct Habit {
    name: String,
    metadata: Option<String>,

    date_begin: NaiveDateTime,
    interval: Duration,

    history: Vec<(NaiveDateTime,    // first: last_date_done
                  NaiveDateTime,    // second: real_last_date_done
                  Option<String>)>, // third: metadata

    //times_missed: i32,
    active: bool
}

//getters
impl Habit {
    pub fn name         (&self) -> &String           { &self.name }
    pub fn metadata     (&self) -> &Option<String>   { &self.metadata }
    pub fn date_begin   (&self) -> &NaiveDateTime    { &self.date_begin }
    pub fn interval     (&self) -> &Duration         { &self.interval }
    pub fn times_done   (&self) -> usize             { self.history.len() }
    //pub fn times_missed (&self) -> i32 {
    //    self.update_times_missed();
    //    self.times_missed
    //}
    pub fn active       (&self) -> bool              { self.active }
    pub fn last_done    (&self) -> Option<&(NaiveDateTime,
                                            NaiveDateTime,
                                            Option<String>)>
                                                     { self.history.last() }

}

//setters
impl Habit {
    pub fn set_name(&mut self, name : &str) {
        self.name.clear();
        self.name = String::from(name);
    }

    pub fn set_metadata(&mut self, data : &str) {
        if data == "" {
            self.metadata = None;
        } else {
            self.metadata = Some(String::from(data));
        }
    }

    pub fn set_date_begin(&mut self, new_date: &NaiveDateTime) -> &NaiveDateTime {
        self.date_begin = new_date.clone();
        &self.date_begin
    }

    pub fn set_interval(&mut self, new_interval: Duration) -> Result<&Duration, &str> {
        if new_interval < Duration::zero() {
            return Err("Unable to have a negative interval");
        }
        self.interval = new_interval;
        Ok(&self.interval)
    }

    pub fn set_active(&mut self, is_active: bool) -> Result<bool, &str> {
        if self.date_begin < Utc::now().naive_utc() {
            self.active = is_active;
            Ok(self.active)
        } else {
            Err("Unable to change active")
        }
    }
}

// métier
impl Habit {

    pub fn next_date_done(&self) -> Result<NaiveDateTime, &'static str> {
        match self.active {
            false => Err("The habit is not active"),
            true => {
                let mut date_done = self.date_begin.clone();
                let now = Utc::now().naive_utc();
                while date_done < now {
                    date_done = date_done + self.interval
                }
                Ok(date_done)
            }
        }
    }

    pub fn has_to_be_done(&self) -> bool {
        match self.active {
            false => false,
            true => {
                let now = Utc::now().naive_utc();
                match self.history.last() {
                    None => {
                        if now > self.date_begin {
                            true
                        } else {
                            false
                        }
                    },
                    Some(last_done) => {
                        if now > last_done.1 {
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }

    pub fn done(&mut self, metadata: Option<String>) -> Result<&(NaiveDateTime, NaiveDateTime, Option<String>), &'static str>{
        if self.active == false {
            return Err("Unable to done an unactive habit");
        }

        if ! self.has_to_be_done() {
            return Err("Unable to done a habit that has already been done");
        }
        let next_date_done = self.next_date_done().unwrap();
        let real_next_date_done = Utc::now().naive_utc();
        self.history.push((next_date_done, real_next_date_done , metadata));
        Ok(&self.history.last().unwrap())
    }

    //pub fn update_times_missed() -> i32 {
    //    let now = Utc::now().naive_utc();
    //    let date = self.date_begin;
    //    while date < self.now {
    //        date += self.interval
    //    }
    //}
}

impl Default for Habit {
    fn default() -> Self {
        let date_begin = Utc::now().naive_utc();
        let interval = Duration::zero();
        let history: Vec<(NaiveDateTime, NaiveDateTime, Option<String>)> = Vec::new();
        let name = String::new();

        Habit {
            name: name,
            metadata: None,

            date_begin: date_begin,
            interval: interval,

            history: history,

            times_missed: 0,
            active: false
        }
    }
}
