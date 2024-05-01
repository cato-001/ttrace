use std::fmt::Display;

use chrono::{Datelike, Days, Local, NaiveDate, Weekday};

pub struct DateFmt {
    date: NaiveDate,
}

impl DateFmt {
    pub fn new(date: NaiveDate) -> Self {
        Self { date }
    }
}

impl Display for DateFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let today = Local::now().date_naive();
        if self.date == today {
            return write!(f, "Today");
        }
        if let Some(yesterday) = today.checked_sub_days(Days::new(1)) {
            if self.date == yesterday {
                return write!(f, "Yesterday");
            }
        };
        if let Some(week) = today.checked_sub_days(Days::new(7)) {
            if self.date > week {
                return write!(f, "{}", weekday(&self.date));
            }
        };
        if let Some(tomorrow) = today.checked_add_days(Days::new(1)) {
            if self.date == tomorrow {
                return write!(f, "Tomorrow");
            }
        };
        write!(f, "{}", self.date.format("%Y.%m.%d"))
    }
}

fn weekday(date: &NaiveDate) -> &'static str {
    match date.weekday() {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
}
