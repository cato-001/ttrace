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
        if is_digit(s) && matches!(s.len(), 1 | 2) {
            let time = NaiveTime::from_hms_opt(u32::from_str(s)?, 0, 0)
                .wrap_err("time values are not right")?;
            return Ok(Self::Time(time));
        }
        if is_digit(s) && matches!(s.len(), 3 | 4) {
            let split_index = s.len() - 2;
            let (hours, minutes) = s.split_at(split_index);
            let time = NaiveTime::from_hms_opt(u32::from_str(hours)?, u32::from_str(minutes)?, 0)
                .wrap_err("time values are not right")?;
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
