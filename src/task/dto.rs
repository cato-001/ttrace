pub use {
    day_with_tasks::DayWithTasks,
    task::{set_task_day, task_from_row, Task},
    task_group::TaskGroup,
};

mod task {
    use std::fmt::Display;

    use chrono::{Local, NaiveTime, TimeDelta};
    use rusqlite::Row;

    use crate::day::{Day, DayReference};

    pub fn task_from_row(row: &Row) -> rusqlite::Result<Task> {
        let id = row.get("id")?;
        let day = row.get("day_id")?;
        let start = row.get("start")?;
        let end = row.get("end")?;
        let description = row.get("description")?;
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

    #[derive(Debug, Clone)]
    pub struct Task {
        id: u64,
        day: DayReference,
        start: NaiveTime,
        end: Option<NaiveTime>,
        description: String,
    }

    impl Task {
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

        pub fn delta(&self) -> Option<TimeDelta> {
            self.end
                .or_else(|| Some(self.day.value()?.time()))
                .map(|end| end - self.start)
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
    use crate::day::Day;

    use super::{task_group::TaskGroup, Task};

    pub struct DayWithTasks {
        day: Day,
        tasks: Vec<Task>,
    }

    impl DayWithTasks {
        pub fn new(day: Day, tasks: Vec<Task>) -> Self {
            Self { day, tasks }
        }

        pub fn day(&self) -> &Day {
            &self.day
        }

        pub fn tasks(&self) -> impl Iterator<Item = &Task> {
            self.tasks.iter()
        }

        pub fn task_groups(&self) -> impl Iterator<Item = TaskGroup> {
            let mut groups = Vec::new();

            for task in self.tasks.iter() {
                if groups
                    .iter_mut()
                    .any(|group: &mut TaskGroup| group.add_task(task.clone()))
                {
                    continue;
                }
                groups.push(TaskGroup::from_task(task.clone()));
            }

            groups.into_iter()
        }
    }
}

mod task_group {
    use chrono::TimeDelta;

    use super::Task;

    pub struct TaskGroup {
        description: String,
        tasks: Vec<Task>,
    }

    impl TaskGroup {
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
    }
}
