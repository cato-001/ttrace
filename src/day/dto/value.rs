use std::fmt::Display;

use chrono::{Local, NaiveDate, NaiveTime};
use rusqlite::Row;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Day {
    id: u64,
    date: NaiveDate,
}

pub trait DayRef {
    fn id(&self) -> u64;
    fn value(&self) -> Option<Day>;
}

impl Day {
    pub fn new(id: u64, date: NaiveDate) -> Self {
        Self { id, date }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn is_today(&self) -> bool {
        self.date == Local::now().date_naive()
    }

    pub fn time(&self) -> NaiveTime {
        if !self.is_today() {
            return NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        }
        Local::now().time()
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "day id={} date={}",
            self.id,
            self.date.format("%Y-%m-%d")
        )
    }
}

impl DayRef for u64 {
    fn id(&self) -> u64 {
        *self
    }

    fn value(&self) -> Option<Day> {
        None
    }
}

impl DayRef for Day {
    fn id(&self) -> u64 {
        self.id
    }

    fn value(&self) -> Option<Day> {
        Some(*self)
    }
}
