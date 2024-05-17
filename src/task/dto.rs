pub use {
    day_with_tasks::DayWithTasks,
    task::{set_task_day, set_task_description, task_from_row, Task},
    task_group::TaskGroup,
};

mod task {
    use std::fmt::Display;

    use chrono::{Local, NaiveTime, TimeDelta};
    use rusqlite::Row;
    use serde::Serialize;

    use crate::day::{Day, DayReference};

    pub fn task_from_row(row: &Row) -> rusqlite::Result<Task> {
        let id = row.get("id")?;
        let day = row.get("day_id")?;
        let start = row.get("start")?;
        let end = row.get("end")?;
        let description: String = row.get("description")?;
        let description = description.trim().to_owned();
        Ok(Task {
            id,
            day,
            start,
            end,
            description,
        })
    }

    pub fn set_task_day(task: &mut Task, day: Day) {
        task.day = DayReference::Value(day);
    }

    pub fn set_task_description(task: &mut Task, description: &str) {
        task.description.clear();
        task.description.push_str(description);
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Task {
        id: u64,
        #[serde(skip)]
        day: DayReference,
        start: NaiveTime,
        end: Option<NaiveTime>,
        description: String,
    }

    impl Task {
        pub fn id(&self) -> u64 {
            self.id
        }

        pub fn day_id(&self) -> u64 {
            match &self.day {
                DayReference::Id(id) => *id,
                DayReference::Value(day) => day.id(),
            }
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

        pub fn end_or_time(&self) -> Option<NaiveTime> {
            self.end.or_else(|| Some(self.day.value()?.time()))
        }

        pub fn is_active(&self) -> bool {
            self.end.is_none()
        }

        pub fn delta(&self) -> Option<TimeDelta> {
            self.end_or_time().map(|end| end - self.start)
        }
    }

    impl Display for Task {
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
}

mod day_with_tasks {
    use chrono::TimeDelta;
    use itertools::Itertools;
    use serde::Serialize;

    use crate::day::Day;

    use super::{task_group::TaskGroup, Task};

    #[derive(Serialize)]
    pub struct DayWithTasks {
        day: Day,
        tasks: Vec<Task>,
    }

    impl DayWithTasks {
        pub fn new(day: Day, tasks: Vec<Task>) -> Self {
            Self { day, tasks }
        }

        pub fn is_empty(&self) -> bool {
            self.tasks.is_empty()
        }

        pub fn delta(&self) -> TimeDelta {
            self.tasks.iter().filter_map(|task| task.delta()).sum()
        }

        pub fn day(&self) -> &Day {
            &self.day
        }

        pub fn tasks(&self) -> impl Iterator<Item = &Task> {
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
}

mod task_group {
    use chrono::{NaiveTime, TimeDelta};

    use super::Task;

    pub struct TaskGroup {
        description: String,
        tasks: Vec<Task>,
    }

    impl TaskGroup {
        pub fn new(description: String, tasks: Vec<Task>) -> Self {
            Self { description, tasks }
        }

        pub fn from_task(task: Task) -> Self {
            let description = task.description().to_owned();
            let tasks = vec![task];
            Self { description, tasks }
        }

        pub fn add_task(&mut self, task: Task) -> bool {
            if task.description() != self.description {
                return false;
            }
            self.tasks.push(task);
            true
        }

        pub fn description(&self) -> &str {
            self.description.as_str()
        }

        pub fn tasks(&self) -> impl Iterator<Item = &Task> {
            self.tasks.iter()
        }

        pub fn delta(&self) -> TimeDelta {
            self.tasks.iter().filter_map(|task| task.delta()).sum()
        }

        pub fn latest_time(&self) -> Option<NaiveTime> {
            self.tasks.iter().fold(None, |latest_time, task| {
                match (latest_time, task.end_or_time()) {
                    (Some(latest_time), Some(task_end)) => (task_end > latest_time)
                        .then_some(task_end)
                        .or(Some(latest_time)),
                    (None, Some(task_end)) => Some(task_end),
                    (Some(latest_time), None) => Some(latest_time),
                    (None, None) => None,
                }
            })
        }
    }
}
