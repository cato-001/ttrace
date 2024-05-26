use std::str::FromStr;

use chrono::{NaiveTime, TimeDelta};
use eyre::{eyre, ContextCompat};

pub enum TimeOrDelta {
    Time(NaiveTime),
    Delta(TimeDelta),
}

impl FromStr for TimeOrDelta {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        if s.len() == 4 && is_digit(s) {
            let hours = u32::from_str(&s[..2])?;
            let minutes = u32::from_str(&s[2..])?;
            let time = NaiveTime::from_hms_opt(hours, minutes, 0);
            let time = time.wrap_err("time values are not right")?;
            return Ok(Self::Time(time));
        };
        if s[..1] == *"+" {
            let minutes = u32::from_str(&s[1..])?;
            let delta = TimeDelta::minutes(minutes as i64);
            return Ok(Self::Delta(delta));
        }
        if s[..1] == *"-" {
            let minutes = u32::from_str(&s[1..])?;
            let delta = TimeDelta::minutes(-(minutes as i64));
            return Ok(Self::Delta(delta));
        }
        Err(eyre!("could not convert string to time: {}", s))
    }
}

fn is_digit(s: &str) -> bool {
    s.chars().all(|char| matches!(char, '0'..='9'))
}
