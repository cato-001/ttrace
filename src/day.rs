use std::rc::Rc;

use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use eyre::{Context, ContextCompat};
use rusqlite::{Connection, Params};

pub use dto::{Day, DayReference};

use self::dto::day_from_row;

mod dto;

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
        let date = Local::now().date_naive();
        self.from_date(&date)
    }

    pub fn yesterday(&self) -> eyre::Result<Day> {
        let date = Local::now()
            .date_naive()
            .checked_sub_days(Days::new(1))
            .wrap_err("could not get yesterdays date!")?;
        self.from_date(&date)
    }

    pub fn week_till_today(&self) -> eyre::Result<Vec<Day>> {
        const ONE_DAY: Days = Days::new(1);
        let mut date = Local::now().date_naive();
        let mut week = Vec::new();

        loop {
            week.push(self.from_date(&date)?);

            if matches!(date.weekday(), Weekday::Mon) {
                break;
            }
            let Some(next) = date.checked_sub_days(ONE_DAY) else {
                break;
            };
            date = next
        }

        Ok(week)
    }

    pub fn list_passed_days(&self, count: usize) -> eyre::Result<Vec<Day>> {
        self.query(
            "SELECT id, date FROM days ORDER BY date DESC LIMIT ?1",
            (count,),
        )
    }

    pub fn from_date(&self, date: &NaiveDate) -> eyre::Result<Day> {
        if let Ok(day) = self.from_date_or_none(&date) {
            return Ok(day);
        }
        self.insert_from_date(&date);
        self.from_date_or_none(&date)
    }

    fn from_date_or_none(&self, date: &NaiveDate) -> eyre::Result<Day> {
        self.get("SELECT id, date FROM days WHERE date = ?1", (date,))
    }

    pub fn resolve(&self, reference: DayReference) -> eyre::Result<Day> {
        match reference {
            DayReference::Id(id) => self.day(id),
            DayReference::Value(day) => Ok(day),
        }
    }

    pub fn day(&self, id: u64) -> eyre::Result<Day> {
        self.get("SELECT id, date FROM days WHERE id = ?1", (id,))
    }

    fn insert_from_date(&self, date: &NaiveDate) -> eyre::Result<()> {
        let _ = self
            .connection
            .execute("INSERT INTO days (date) VALUES (?1)", (date,))?;
        Ok(())
    }
}

impl DayRepository {
    fn get(&self, statement: &str, parameters: impl Params) -> eyre::Result<Day> {
        self.connection
            .query_row(statement, parameters, day_from_row)
            .wrap_err("could not query day")
            .with_context(|| statement.to_owned())
    }

    fn query(&self, query: &str, parameters: impl Params) -> eyre::Result<Vec<Day>> {
        self.connection
            .prepare(query)?
            .query_map(parameters, day_from_row)
            .wrap_err("could not execute sql statement")
            .with_context(|| query.to_owned())?
            .into_iter()
            .collect::<Result<_, _>>()
            .wrap_err("cannot convert tasks from sql statement")
            .with_context(|| query.to_owned())
    }
}
