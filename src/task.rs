use std::rc::Rc;

use chrono::{Local, NaiveTime};
use eyre::{eyre, Context};
use rusqlite::{Connection, Params};

pub use dto::{DayWithTasks, Task, TaskGroup};

use crate::day::Day;

use self::dto::{set_task_day, task_from_row};

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
        for task in tasks.iter_mut() {
            set_task_day(task, day.clone());
        }
        Ok(DayWithTasks::new(day, tasks))
    }

    pub fn start(&self, day: &Day, description: &str) -> eyre::Result<Task> {
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

    pub fn stop(&self, day: &Day) -> eyre::Result<u64> {
        let current = self
            .current(day)
            .wrap_err("a current task is needed to stop it")
            .with_context(|| format!("{:?}", day))?;
        if current.end().is_some() {
            let error = Err(eyre!("task already has an end!"));
            return error.with_context(|| format!("{:?}", current));
        }
        let now = Local::now().time();
        self.connection
            .execute("UPDATE tasks SET end=?1 WHERE id=?2", (now, current.id()))
            .wrap_err("could not end task")
            .with_context(|| format!("{:?}", current))?;
        Ok(current.id())
    }

    pub fn current(&self, day: &Day) -> eyre::Result<Task> {
        let mut task = self.get(
            "SELECT id, day_id, start, end, description
             FROM tasks
             WHERE day_id=?1 AND end IS null",
            (day.id(),),
        )?;
        set_task_day(&mut task, day.clone());
        Ok(task)
    }

    pub fn task(&self, id: u64) -> eyre::Result<Task> {
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

    fn get(&self, query: &str, params: impl Params) -> eyre::Result<Task> {
        self.connection
            .query_row(query, params, task_from_row)
            .wrap_err("could not execute sql statement")
            .with_context(|| query.to_owned())
    }

    fn query(&self, query: &str, params: impl Params) -> eyre::Result<Vec<Task>> {
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
}
