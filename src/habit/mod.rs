use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Weekday, Utc};
use habittools::*;

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
