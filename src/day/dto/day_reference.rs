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

    pub fn id(&self) -> u64 {
        match self {
            DayReference::Id(id) => *id,
            DayReference::Value(day) => day.id(),
        }
    }
}

impl FromSql for DayReference {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
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
