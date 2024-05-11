pub use {
    day::{day_from_row, Day},
    day_reference::DayReference,
};

mod day {
    use std::fmt::Display;

    use chrono::{Local, NaiveDate, NaiveTime};
    use rusqlite::Row;
    use serde::Serialize;

    pub fn day_from_row(row: &Row) -> rusqlite::Result<Day> {
        let id = row.get("id")?;
        let date = row.get("date")?;
        Ok(Day { id, date })
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Day {
        id: u64,
        date: NaiveDate,
    }

    impl Day {
        pub fn id(&self) -> u64 {
            self.id
        }

        pub fn date(&self) -> NaiveDate {
            self.date
        }

        pub fn is_today(&self) -> bool {
            self.date == Local::now().date_naive()
        }

        pub fn time(&self) -> NaiveTime {
            if !self.is_today() {
                return NaiveTime::from_hms_opt(23, 59, 59).unwrap();
            }
            Local::now().time()
        }
    }

    impl Display for Day {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "day id={} date={}",
                self.id,
                self.date.format("%Y-%m-%d")
            )
        }
    }
}

mod day_reference {
    use rusqlite::{
        types::{FromSql, ToSqlOutput, Value},
        ToSql,
    };
    use serde::Serialize;

    use super::Day;

    #[derive(Debug, Clone, Serialize)]
    pub enum DayReference {
        Id(u64),
        Value(Day),
    }

    impl DayReference {
        pub fn value(&self) -> Option<&Day> {
            match self {
                Self::Id(_) => None,
                Self::Value(day) => Some(day),
            }
        }
    }

    impl FromSql for DayReference {
        fn column_result(
            value: rusqlite::types::ValueRef<'_>,
        ) -> rusqlite::types::FromSqlResult<Self> {
            let id = value.as_i64()?;
            Ok(Self::Id(id as u64))
        }
    }

    impl ToSql for DayReference {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let id = match self {
                DayReference::Id(id) => *id,
                DayReference::Value(day) => day.id(),
            };
            Ok(ToSqlOutput::Owned(Value::Integer(id as i64)))
        }
    }
}
