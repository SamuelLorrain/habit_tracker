use chrono::{NaiveDate, Weekday, Utc};

use num_traits::cast::FromPrimitive;

// use generic instead of NaiveDate ?

pub fn get_current_weekday() -> Weekday {
    Weekday::from_usize(
        Utc::now()
        .naive_utc()
        .format("%u")
        .to_string()
        .parse::<usize>()
        .unwrap() - 1 as usize
    ).unwrap()
}

pub fn get_weekday(date: NaiveDate) -> Weekday {
    Weekday::from_usize(
        date
        .format("%u")
        .to_string()
        .parse::<usize>()
        .unwrap() - 1 as usize
    ).unwrap()
}

pub fn get_monthday(date: NaiveDate) -> usize {
    date.format("%d").to_string().parse::<usize>().unwrap()
}

pub fn get_next_date_with_weekday(date: &NaiveDate, weekday: Weekday) -> NaiveDate {
    let mut next_date = date.clone();

    if get_weekday(next_date) == weekday {
        next_date = next_date.succ();
    }

    while get_weekday(next_date) != weekday {
        next_date = next_date.succ();
    }
    return next_date;
}

pub fn get_next_date_with_monthday(date: &NaiveDate, month_day: usize) -> NaiveDate {
    let mut next_date = date.clone();

    if get_monthday(next_date) == month_day{
        next_date = next_date.succ();
    }

    while get_monthday(next_date) != month_day {
        next_date = next_date.succ();
    }
    return next_date;
}

pub fn parse_weekday(day: &str) -> Result<Weekday, &str> {
    match day.to_lowercase().as_ref() {
        "mon" | "monday"    => Ok(Weekday::Mon),
        "tue" | "tuesday"   => Ok(Weekday::Tue),
        "wed" | "wednesday" => Ok(Weekday::Wed),
        "thu" | "thursday"  => Ok(Weekday::Thu),
        "fri" | "friday"    => Ok(Weekday::Fri),
        "sat" | "saturday"  => Ok(Weekday::Sat),
        "sun" | "sunday"    => Ok(Weekday::Sun),
        _ => Err("String is not a day")
    }
}
