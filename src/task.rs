use std::rc::Rc;

use chrono::{Local, NaiveTime, TimeDelta};
use eyre::{eyre, Context};
use rusqlite::{Connection, Params, Row};

pub use dto::{DayWithTasks, Task, TaskGroup};

use crate::day::{Day, DayRef};

use self::dto::MutTask;

mod dto;

pub struct TaskRepository {
    connection: Rc<Connection>,
}

impl TaskRepository {
    pub fn day_with_tasks(&self, day: Day) -> eyre::Result<DayWithTasks> {
        let mut tasks = self
            .query(
                "SELECT id, day_id, start, end, description FROM tasks WHERE day_id=?1",
                (day.id(),),
            )
            .with_context(|| format!("cannot query tasks for day: {:?}", day))?;
        let tasks = tasks
            .into_iter()
            .map(|task| MutTask::with_day(task, day))
            .collect();
        Ok(DayWithTasks::new(day, tasks))
    }

    pub fn start(&self, day: Day, description: &str) -> eyre::Result<Task<Day>> {
        let desciption = description.trim();
        if self.current(day).is_ok() {
            self.stop(day)
                .with_context(|| "could not end the current task before starting a new one.")?;
        };
        let now = Local::now().time();
        self.connection
            .execute(
                "INSERT INTO tasks (day_id, start, end, description)
                 VALUES (?1, ?2, null, ?3)",
                (day.id(), now, description),
            )
            .wrap_err("could not start a new task")
            .with_context(|| description.to_owned())?;
        self.current(day)
            .wrap_err("could not get newly created task")
    }

    pub fn stop(&self, day: Day) -> eyre::Result<Task<Day>> {
        let mut current = self
            .current(day)
            .wrap_err("a current task is needed to stop it")
            .with_context(|| format!("{:?}", day))?;
        if current.end().is_some() {
            let error = Err(eyre!("task already has an end!"));
            return error.with_context(|| format!("{:?}", current));
        }
        let now = Local::now().time();
        MutTask::set_end(&mut current, now);
        self.save(&current);
        Ok(current)
    }

    pub fn rename_current(&self, day: Day, description: &str) -> eyre::Result<Task<Day>> {
        let task = self.current(day)?;
        self.rename_task(task, description)
    }

    pub fn rename_task<DayRefImpl>(
        &self,
        mut task: Task<DayRefImpl>,
        description: &str,
    ) -> eyre::Result<Task<DayRefImpl>>
    where
        DayRefImpl: DayRef,
    {
        MutTask::set_description(&mut task, description);
        self.save(&task)?;
        Ok(task)
    }

    pub fn shift_start(&self, task: Task<Day>, delta: TimeDelta) -> eyre::Result<Task<Day>> {
        let time = task.start() + delta;
        self.set_start(task, time)
    }

    pub fn set_start(&self, mut task: Task<Day>, time: NaiveTime) -> eyre::Result<Task<Day>> {
        if time == task.start() {
            return Ok(task);
        }
        if time > task.start() {
            if time >= task.end_or_day_time() {
                return Err(eyre!("cannot set start past the end time"));
            }
        }
        if let Some(prev) = self.prev(&task)? {
            let prev_end = prev.end_or_day_time();
            if prev_end == task.start() || prev_end > time {
                _ = self.set_end(prev, time)?;
            }
        }
        MutTask::set_start(&mut task, time);
        self.save(&task)?;
        Ok(task)
    }

    pub fn set_end(&self, mut task: Task<Day>, time: NaiveTime) -> eyre::Result<Task<Day>> {
        if time <= task.start() {
            return Err(eyre!(
                "cannot set end time before start time: {} <= {}",
                time,
                task.start()
            ));
        }
        MutTask::set_end(&mut task, time);
        self.save(&task)?;
        Ok(task)
    }

    pub fn current(&self, day: Day) -> eyre::Result<Task<Day>> {
        let task = self.get(
            "SELECT id, day_id, start, end, description
             FROM tasks
             WHERE day_id=?1 AND end IS null",
            (day.id(),),
        )?;
        Ok(MutTask::with_day(task, day))
    }

    pub fn prev(&self, task: &Task<Day>) -> eyre::Result<Option<Task<Day>>> {
        let prev = self
            .get_opt(
                "SELECT id, day_id, start, end, description
                 FROM tasks
                 WHERE day_id=?1 AND end <= ?2
                 ORDER BY end DESC
                 LIMIT 1",
                (task.day_id(), task.start()),
            )?
            .map(|prev| MutTask::with_day(prev, task.day()));
        Ok(prev)
    }

    pub fn task(&self, id: u64) -> eyre::Result<Task<u64>> {
        self.get(
            "SELECT id, day_id, start, end, description
             FROM tasks
             WHERE id=?1",
            (id,),
        )
    }
}

impl TaskRepository {
    pub fn new(connection: Rc<Connection>) -> eyre::Result<Self> {
        let _ = connection.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                day_id INTEGER NOT NULL,
                start DATE NOT NULL,
                end DATE,
                description TEXT NOT NULL
            )",
            (),
        )?;
        Ok(Self { connection })
    }

    fn get_opt(&self, query: &str, params: impl Params) -> eyre::Result<Option<Task<u64>>> {
        let tasks = self.query(query, params)?;
        if tasks.len() == 0 {
            return Ok(None);
        }
        if tasks.len() == 1 {
            let task = &tasks[0];
            return Ok(Some(task.clone()));
        }
        Err(eyre!("to many elements"))
    }

    fn get(&self, query: &str, params: impl Params) -> eyre::Result<Task<u64>> {
        self.connection
            .query_row(query, params, task_from_row)
            .wrap_err("could not execute sql statement")
            .with_context(|| query.to_owned())
    }

    fn query(&self, query: &str, params: impl Params) -> eyre::Result<Vec<Task<u64>>> {
        self.connection
            .prepare(query)?
            .query_map(params, task_from_row)
            .wrap_err("could not execute sql statement")
            .with_context(|| query.to_owned())?
            .into_iter()
            .collect::<Result<_, _>>()
            .wrap_err("cannot convert tasks from sql statement")
            .with_context(|| query.to_owned())
    }

    fn save(&self, task: &Task<impl DayRef>) -> eyre::Result<()> {
        self.connection.execute(
            "UPDATE tasks SET day_id=?1, start=?2, end=?3, description=?4 WHERE id=?5",
            (
                task.day_id(),
                task.start(),
                task.end(),
                task.description(),
                task.id(),
            ),
        )?;
        Ok(())
    }
}

pub fn task_from_row(row: &Row) -> rusqlite::Result<Task<u64>> {
    let id = row.get("id")?;
    let day = row.get("day_id")?;
    let start = row.get("start")?;
    let end = row.get("end")?;
    let description: String = row.get("description")?;
    let description = description.trim().to_owned();
    Ok(Task::new(id, day, start, end, description))
}
