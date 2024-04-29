use std::fmt::Display;

use chrono::{Days, Local, NaiveDate};

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
        if let Some(tomorrow) = today.checked_add_days(Days::new(1)) {
            if self.date == tomorrow {
                return write!(f, "Yesterday");
            }
        };
        write!(f, "{}", self.date.format("%Y.%m.%d"))
    }
}
