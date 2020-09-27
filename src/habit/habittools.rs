#[derive(Debug)]
pub struct HabitHistoryItem {
    datetime_done: NaiveDateTime,
    metadata: Option<String>,
}

#[derive(Debug)]
pub enum EndRepeatType {
    Never,
    On(NaiveDate),
    AfterOccurences(usize)
}

#[derive(Debug)]
pub enum RepeatTimeUnit {
    Days,
    Weeks,
    Months,
    Years
}

#[derive(Debug)]
pub enum RepeatMonth {
    DayOfMonth(usize),
    DayOfWeek(usize, Weekday)
}

#[derive(Debug)]
pub struct RepeatHabit {
    end_type: EndRepeatType,
    time_unit: RepeatTimeUnit,
    time: usize,
    weekdays: Option<Vec<Weekday>>,
    repeat_month: Option<RepeatMonth>
}

