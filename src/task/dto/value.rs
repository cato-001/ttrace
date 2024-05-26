use std::fmt::Display;

use chrono::{Local, NaiveTime, TimeDelta};
use rusqlite::Row;
use serde::Serialize;

use crate::day::{Day, DayRef, DayReference};

#[derive(Debug, Clone, Serialize)]
pub struct Task<DayRefImpl> {
    id: u64,
    #[serde(skip)]
    day: DayRefImpl,
    start: NaiveTime,
    end: Option<NaiveTime>,
    description: String,
}

pub struct MutTask {}

impl<DayRefImpl> Task<DayRefImpl> {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn start(&self) -> NaiveTime {
        self.start
    }

    pub fn end(&self) -> Option<NaiveTime> {
        self.end
    }

    pub fn is_active(&self) -> bool {
        self.end.is_none()
    }
}

impl<DayRefImpl> Task<DayRefImpl>
where
    DayRefImpl: DayRef,
{
    pub fn new(
        id: u64,
        day: DayRefImpl,
        start: NaiveTime,
        end: Option<NaiveTime>,
        description: String,
    ) -> Self {
        Self {
            id,
            day,
            start,
            end,
            description,
        }
    }

    pub fn day_id(&self) -> u64 {
        self.day.id()
    }
}

impl Task<u64> {
    pub fn day(&self) -> u64 {
        self.day
    }

    pub fn delta(&self) -> Option<TimeDelta> {
        self.end.map(|end| end - self.start)
    }
}

impl Task<Day> {
    pub fn day(&self) -> Day {
        self.day
    }

    pub fn end_or_day_time(&self) -> NaiveTime {
        self.end.unwrap_or_else(|| self.day.time())
    }

    pub fn delta(&self) -> TimeDelta {
        self.end_or_day_time() - self.start
    }
}

impl<DayRefImpl> Display for Task<DayRefImpl> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "task \"{}\" id={} start={} end=",
            self.description,
            self.id,
            self.start.format("%H:%M")
        )?;
        let Some(end) = self.end else {
            return write!(f, "...");
        };
        write!(f, "{}", end.format("%H:%M"))
    }
}

impl MutTask {
    pub(crate) fn with_day(task: Task<u64>, day: Day) -> Task<Day> {
        Task::new(task.id, day, task.start, task.end, task.description)
    }

    pub(crate) fn set_description<DayRefImpl>(task: &mut Task<DayRefImpl>, description: &str) {
        task.description.clear();
        task.description.push_str(description);
    }

    pub(crate) fn set_start(task: &mut Task<impl DayRef>, time: NaiveTime) {
        task.start = time;
    }

    pub(crate) fn set_end(task: &mut Task<impl DayRef>, time: NaiveTime) {
        task.end = Some(time);
    }
}
