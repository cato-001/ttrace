use chrono::TimeDelta;
use itertools::Itertools;
use serde::Serialize;

use crate::day::Day;

use super::{task_group::TaskGroup, Task};

#[derive(Serialize)]
pub struct DayWithTasks {
    day: Day,
    tasks: Vec<Task<Day>>,
}

impl DayWithTasks {
    pub fn new(day: Day, tasks: Vec<Task<Day>>) -> Self {
        Self { day, tasks }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn delta(&self) -> TimeDelta {
        self.tasks.iter().map(|task| task.delta()).sum()
    }

    pub fn day(&self) -> &Day {
        &self.day
    }

    pub fn tasks(&self) -> impl Iterator<Item = &Task<Day>> {
        self.tasks.iter()
    }

    pub fn task_groups(&self) -> Vec<TaskGroup> {
        let mut groups: Vec<_> = self
            .tasks
            .iter()
            .cloned()
            .into_group_map_by(|task| task.description().to_owned())
            .into_iter()
            .map(|(key, value)| TaskGroup::new(key, value))
            .collect();

        groups.sort_by_key(|group| group.latest_time());

        groups
    }
}
