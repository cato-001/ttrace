use std::rc::Rc;

use chrono::{Local, NaiveDate};
use eyre::Context;
use rusqlite::{Connection, Params, Row};

#[derive(Debug)]
pub struct Day {
    id: u64,
    date: NaiveDate,
}

impl Day {
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Day {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let id = row.get("id")?;
        let date = row.get("date")?;
        Ok(Day { id, date })
    }
}

pub struct DayRepository {
    connection: Rc<Connection>,
}

impl DayRepository {
    pub fn new(connection: Rc<Connection>) -> eyre::Result<Self> {
        let _ = connection.execute(
            "CREATE TABLE IF NOT EXISTS days (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date DATE NOT NULL
            )",
            (),
        )?;
        Ok(Self { connection })
    }

    pub fn today(&self) -> eyre::Result<Day> {
        let now = Local::now().date_naive();
        if let Ok(today) = self.get_from_date(&now) {
            return Ok(today);
        };
        self.insert_from_date(&now)?;
        return self.get_from_date(&now);
    }

    pub fn list_passed_days(&self, count: usize) -> eyre::Result<Vec<Day>> {
        self.query(
            "SELECT id, date FROM days ORDER BY date DESC LIMIT ?1",
            (count,),
        )
    }

    pub fn get_from_date(&self, date: &NaiveDate) -> eyre::Result<Day> {
        self.get("SELECT id, date FROM days WHERE date = ?1", (date,))
    }

    fn insert_from_date(&self, date: &NaiveDate) -> eyre::Result<()> {
        let _ = self
            .connection
            .execute("INSERT INTO days (date) VALUES (?1)", (date,))?;
        Ok(())
    }

    fn get(&self, statement: &str, parameters: impl Params) -> eyre::Result<Day> {
        self.connection
            .query_row(statement, parameters, Day::from_row)
            .wrap_err("could not query day")
            .with_context(|| statement.to_owned())
    }

    fn query(&self, query: &str, parameters: impl Params) -> eyre::Result<Vec<Day>> {
        self.connection
            .prepare(query)?
            .query_map(parameters, Day::from_row)
            .wrap_err("could not execute sql statement")
            .with_context(|| query.to_owned())?
            .into_iter()
            .collect::<Result<_, _>>()
            .wrap_err("cannot convert tasks from sql statement")
            .with_context(|| query.to_owned())
    }
}
