pub use day::{day_from_row, Day};

mod day {
    use std::fmt::Display;

    use chrono::{Local, NaiveDate};
    use rusqlite::Row;

    pub fn day_from_row(row: &Row) -> rusqlite::Result<Day> {
        let id = row.get("id")?;
        let date = row.get("date")?;
        Ok(Day { id, date })
    }

    #[derive(Debug, Clone)]
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
