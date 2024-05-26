use chrono::{NaiveTime, TimeDelta};

use crate::day::Day;

use super::Task;

pub struct TaskGroup {
    description: String,
    tasks: Vec<Task<Day>>,
}

impl TaskGroup {
    pub fn new(description: String, tasks: Vec<Task<Day>>) -> Self {
        Self { description, tasks }
    }

    pub fn from_task(task: Task<Day>) -> Self {
        let description = task.description().to_owned();
        let tasks = vec![task];
        Self { description, tasks }
    }

    pub fn add_task(&mut self, task: Task<Day>) -> bool {
        if task.description() != self.description {
            return false;
        }
        self.tasks.push(task);
        true
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn tasks(&self) -> impl Iterator<Item = &Task<Day>> {
        self.tasks.iter()
    }

    pub fn delta(&self) -> TimeDelta {
        self.tasks.iter().map(|task| task.delta()).sum()
    }

    pub fn latest_time(&self) -> Option<NaiveTime> {
        self.tasks.iter().fold(None, |latest_time, task| {
            let task_end = task.end_or_day_time();
            match latest_time {
                Some(latest_time) => {
                    if task_end > latest_time {
                        Some(task_end)
                    } else {
                        Some(latest_time)
                    }
                }
                None => Some(task_end),
            }
        })
    }
}
