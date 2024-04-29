use std::fmt::Display;

use chrono::NaiveTime;

pub struct TimeFmt {
    time: Option<NaiveTime>,
}

impl TimeFmt {
    pub fn new(time: NaiveTime) -> Self {
        let time = Some(time);
        Self { time }
    }
    pub fn option(time: Option<NaiveTime>) -> Self {
        Self { time }
    }
}

impl Display for TimeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(time) = self.time else {
            return write!(f, "...");
        };
        write!(f, "{}", time.format("%H:%M"))
    }
}
