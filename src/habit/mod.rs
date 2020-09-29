pub mod habittools;
pub mod datetools;

use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Weekday, Utc, Duration};
use habittools::*;
use datetools::*;

use crate::habit::EndRepeatType::*;
use crate::habit::RepeatTimeUnit::*;
use crate::habit::RepeatMonth::*;

#[derive(Debug)]
pub struct Habit {
    name: String,
    metadata: Option<String>,

    date_begin: NaiveDate,
    time_habit: Option<NaiveTime>,

    history: Vec<HabitHistoryItem>,

    end_type: EndRepeatType,
    time_unit: RepeatTimeUnit,
    time_repeat: usize,
    weekdays: Option<Vec<Weekday>>,
    repeat_month: Option<RepeatMonth>,
}

// getters
impl Habit {
    pub fn name(&self)            -> &String                { &self.name }
    pub fn metadata(&self)        -> &Option<String>        { &self.metadata }
    pub fn date_begin(&self)      -> &NaiveDate             { &self.date_begin }
    pub fn time_habit(&self)      -> &Option<NaiveTime>     { &self.time_habit }
    pub fn history(&self)         -> &Vec<HabitHistoryItem> { &self.history }
    pub fn end_type(&self)        -> &EndRepeatType         { &self.end_type }
    pub fn time_unit(&self)       -> &RepeatTimeUnit        { &self.time_unit }
    pub fn time_repeat(&self)     -> usize                  { self.time_repeat }
    pub fn weekdays(&self)        -> &Option<Vec<Weekday>>  { &self.weekdays }
    pub fn repeat_month(&self)    -> &Option<RepeatMonth>   { &self.repeat_month }
}

// setters
impl Habit {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_metadata(&mut self, metadata: &Option<String>) {
        self.metadata = metadata.clone();
    }
    pub fn set_date_begin(&mut self, date_begin: &NaiveDate) {
        self.date_begin = date_begin.clone();
    }
    pub fn set_time_habit(&mut self, time_habit: &Option<NaiveTime>) {
        self.time_habit = time_habit.clone();
    }
    pub fn set_end_type(&mut self, end_type: &EndRepeatType) {
        self.end_type = end_type.clone();
    }
    pub fn set_time_unit(&mut self, time_unit: &RepeatTimeUnit) {
        self.time_unit = time_unit.clone();
    }
    pub fn set_time_repeat(&mut self, time_repeat: usize) {
        self.time_repeat = time_repeat;
    }
    pub fn set_weekdays(&mut self, weekdays: &Option<Vec<Weekday>>) {
        self.weekdays = weekdays.clone();
    }
    pub fn set_repeat_month(&mut self, repeat_month: &Option<RepeatMonth>) {
        self.repeat_month = repeat_month.clone();
    }
    pub fn push_history(&mut self, datetime_done: &NaiveDateTime, metadata: &Option<String> ) {
        self.history.push(HabitHistoryItem::new(
            datetime_done,
            metadata
        ));
    }
}

impl Habit {
    pub fn show(&self) {
        println!("name: {}", self.name);
        match &self.metadata {
            None => (),
            Some(x) => println!("metadata: {}", x)
        }
        print!("The habit started {}", self.date_begin);
        match &self.time_habit {
            None => (),
            Some(x) => print!(" - {}", x)
        }
        print!("\n");

        match self.end_type {
            Never => (),
            On(x) => println!("The habit should stop on {}", x),
            AfterOccurrences(x) => println!("The habit should stop after {} occurrences", x)
        }

        match self.time_unit {
            Days => println!("The habit should be repeated every {} days", self.time_repeat),
            Weeks => println!("The habit should be repeated every {} weeks", self.time_repeat),
            Months => println!("The habit should be repeated {} time(s) every months", self.time_repeat),
            Years => println!("The habit should be repeated {} time(s) every years", self.time_repeat),
        }

        if self.time_unit == Weeks {
            match &self.weekdays {
                None => (),
                Some(weekdays) => {
                    print!("The habit should be repeated on ");
                    for x in weekdays {
                        print!("{} ", x);
                    }
                    print!("\n");
                }
            }
        }

        if self.time_unit == Months {
            match &self.repeat_month {
                None => (),
                Some(month_type) => {
                    match month_type {
                        DayOfMonth(x) => println!("The habit should be repeated the {} of the month", x),
                        DayOfWeek(x, day) => println!("The habit should be repeated the {} {} of the month", x, day),
                    }
                }
            }
        }
    }

    pub fn date_iter(&self) -> HabitDateIter {
        HabitDateIter {
            date_begin: self.date_begin.clone(),
            time_unit: self.time_unit.clone(),
            time_repeat: self.time_repeat.clone(),

            weekdays: self.weekdays.clone(),
            repeat_month: self.repeat_month.clone(),
            next_occurrence: None
        }
    }
}

impl Default for Habit {
    fn default() -> Self {
        let history = Vec::new();
        let date_begin = Utc::now().naive_utc().date();
        let name = String::new();
        Habit {
            name: name,
            metadata: None,

            date_begin: date_begin,
            time_habit: None,

            history: history,

            end_type: EndRepeatType::Never,
            time_unit: RepeatTimeUnit::Days,
            time_repeat: 1,
            weekdays: None,
            repeat_month: None,
        }
    }
}

pub struct HabitDateIter {
    date_begin: NaiveDate,
    time_unit: RepeatTimeUnit,
    time_repeat: usize,

    weekdays: Option<Vec<Weekday>>,
    repeat_month: Option<RepeatMonth>,

    next_occurrence: Option<NaiveDate>
}

impl Iterator for HabitDateIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time_unit {
            Days => {
                if self.next_occurrence == None {
                    self.next_occurrence = Some(self.date_begin);
                    return self.next_occurrence;
                }
                self.next_occurrence = Some(
                    self.next_occurrence.unwrap() + Duration::days(self.time_repeat as i64));
                return self.next_occurrence;
            },
            Weeks => {
                match &self.weekdays {
                    None => None,
                    Some(weekdays) => {
                        if self.next_occurrence == None {
                            self.next_occurrence = Some(self.date_begin.pred());
                        }

                        let date = weekdays.iter()
                            .map(|x| get_next_date_with_weekday(&self.next_occurrence.unwrap(),*x))
                            .min()
                            .unwrap();
                        self.next_occurrence = Some(date);
                        self.next_occurrence
                    }
                }
            },
            Months => {
                match &self.repeat_month {
                    None => None,
                    Some(month) => match month {
                        DayOfMonth(x) => {
                            if self.next_occurrence == None {
                                self.next_occurrence = Some(self.date_begin.pred());
                            }
                            let date = get_next_date_with_monthday(
                                &self.next_occurrence.unwrap(), *x);
                            self.next_occurrence = Some(date);
                            self.next_occurrence
                        },
                        DayOfWeek(x, day) => {
                            // To be handled !
                            None
                        }
                    }

                }
            },
            _ => None
        }
    }
}
