use chrono::{NaiveDate, NaiveDateTime, Weekday};

#[derive(Debug, Clone)]
pub struct HabitHistoryItem {
    datetime_done: NaiveDateTime,
    metadata: Option<String>,
}

// getters
impl HabitHistoryItem {
    pub fn datetime_done(&self) -> &NaiveDateTime { &self.datetime_done }
    pub fn metadata(&self) -> &Option<String> { &self.metadata }
}

// setters
impl HabitHistoryItem {
    pub fn new(datetime_done: &NaiveDateTime, metadata: &Option<String>) -> HabitHistoryItem {
        HabitHistoryItem {
            datetime_done: datetime_done.clone(),
            metadata: metadata.clone(),
        }
    }

    pub fn set_datetime_done(&mut self, datetime_done: NaiveDateTime) {
        self.datetime_done = datetime_done;
    }
    pub fn set_metadata(&mut self, metadata: Option<String> ) {
        self.metadata = metadata;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EndRepeatType {
    Never,
    On(NaiveDate),
    AfterOccurrences(usize)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RepeatTimeUnit {
    Days,
    Weeks,
    Months,
    Years
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RepeatMonth {
    DayOfMonth(usize),
    DayOfWeek(usize, Weekday)
}

#[derive(Debug, PartialEq)]
pub enum HabitInfo {
    AlreadyDoneToday,
    TodoToday,
    NotDueToday,
}

