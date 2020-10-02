pub mod habit;
pub mod cli;

use chrono::{DateTime, FixedOffset};
use habit::{Habit};
use cli::*;

#[derive(Debug)]
struct Sprint {
    name: Option<String>,
    habits: Vec<Habit>,
    date_begin: DateTime<FixedOffset>,
    date_end: DateTime<FixedOffset>,
    metadata: Option<String>
}

fn main() {
    handle_cli();
}
