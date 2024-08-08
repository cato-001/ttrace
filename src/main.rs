#![allow(unused)]

use std::process::exit;
use std::str::FromStr;

use chrono::{Days, Local, TimeDelta, Timelike};
use clap::{Arg, ArgAction, Command};
use config::Config;
use database::open_database_connection;
use day::DayRepository;
use eyre::{eyre, ContextCompat};
use termfmt::{TermFmtExt, TermFmtsExt};

use crate::task::TaskRepository;

use self::output::{DataBundle, OutputFmt};
use self::time::TimeOrDelta;

mod config;
mod database;
mod day;
mod output;
mod task;
mod time;

fn main() -> eyre::Result<()> {
    let cli = Command::new("ttrack")
        .subcommands([
            Command::new("start")
                .args([
                    Arg::new("description")
                        .num_args(1)
                        .required(true)
                        .help("name of the task"),
                    Arg::new("tags")
                        .long("tags")
                        .short('t')
                        .num_args(1..)
                        .help("tags of the task (may be used to associate projects)"),
                ])
                .about("start a new task, if another task is running it will get stopped"),
            Command::new("stop").about("stop the currently running task"),
            Command::new("rename")
                .arg(
                    Arg::new("description")
                        .num_args(1)
                        .help("the new name of the currently running task"),
                )
                .about("rename the current task."),
            Command::new("restart")
                .arg(
                    Arg::new("time")
                        .num_args(1)
                        .required(true)
                        .allow_negative_numbers(true)
                        .help("the new start time of the currently running task"),
                )
                .about("rename the current task."),
            Command::new("edit")
                .args([
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .help("new name of the task"),
                    Arg::new("tags")
                        .long("tags")
                        .short('t')
                        .num_args(1..)
                        .help("new tags of the task (replaces all old ones)"),
                ])
                .about("edit the currently running task"),
            Command::new("get").about("get the currently running task"),
            Command::new("today").about("list the tasks of today"),
            Command::new("yesterday").about("list the task of yesterday"),
            Command::new("day")
                .arg(
                    Arg::new("days")
                        .num_args(1)
                        .allow_negative_numbers(true)
                        .default_value("0")
                        .help("number of days to go back"),
                )
                .about("list the task of the day"),
            Command::new("week")
                .arg(
                    Arg::new("weeks")
                        .num_args(1)
                        .allow_negative_numbers(true)
                        .default_value("0")
                        .help("number of weeks to go back"),
                )
                .about("list the task of the week"),
            Command::new("is_active").about("exit successfully if a task is currently running"),
        ])
        .about("track the time you spend on projects or other tasks")
        .subcommand_required(true)
        .termfmts()
        .get_matches();

    let config = Config::load()?;
    let connection = open_database_connection(&config)?;

    let mut term = cli.termfmt(&config);

    let day_repository = DayRepository::new(connection.clone())?;
    let task_repository = TaskRepository::new(connection)?;

    match cli.subcommand().unwrap() {
        ("start", command) => {
            let description: &String = command.get_one("description").unwrap();
            let today = day_repository.today()?;
            let task = task_repository.start(today, description.as_str())?;
            term.task(&task);
        }
        ("stop", _) => {
            let today = day_repository.today()?;
            let Ok(task) = task_repository.stop(today) else {
                term.error("no task is started yet!");
                term.end();
                return Ok(());
            };
            term.task(&task);
        }
        ("rename", command) => {
            let description: &String = command.get_one("description").unwrap();
            let today = day_repository.today()?;
            let task = task_repository.rename_current(today, description)?;
            term.task(&task);
        }
        ("restart", command) => {
            let time: &String = command.get_one("time").unwrap();
            let time_or_delta = TimeOrDelta::from_str(&time)?;
            let day = day_repository.today()?;
            let task = task_repository.current(day)?;
            let task = match time_or_delta {
                TimeOrDelta::Time(time) => task_repository.set_start(task, time),
                TimeOrDelta::Delta(delta) => task_repository.shift_start(task, delta),
            }?;
            term.task(&task);
        }
        ("today", _) => {
            let Ok(today) = day_repository.today() else {
                term.error("could not find or create day!");
                term.end();
                return Ok(());
            };
            let tasks_for_day = task_repository.day_with_tasks(today)?;
            term.day_with_tasks(&tasks_for_day);
        }
        ("yesterday", _) => {
            let Ok(yesterday) = day_repository.yesterday().map_err(|error| {
                term.error(format_args!("could not find or create day: {}", error));
                term.end();
            }) else {
                return Ok(());
            };
            let tasks_for_day = task_repository.day_with_tasks(yesterday)?;
            term.day_with_tasks(&tasks_for_day);
        }
        ("day", command) => {
            let days: &String = command.get_one("days").unwrap();
            let days = i32::from_str(days)?;
            let date = Local::now()
                .date_naive()
                .checked_sub_days(Days::new(days.abs() as u64))
                .wrap_err("cannot sub days")?;
            let day = day_repository.from_date(date)?;
            let day_with_tasks = task_repository.day_with_tasks(day)?;
            term.day_with_tasks(&day_with_tasks);
        }
        ("week", command) => {
            let weeks: &String = command.get_one("weeks").unwrap();
            let weeks = i32::from_str(weeks)?;
            let week = if weeks == 0 {
                day_repository.week_till_today()?
            } else {
                let weeks = weeks.abs() as u64;
                let today = Local::now().date_naive();
                let date = today
                    .checked_sub_days(Days::new(weeks * 7))
                    .unwrap_or(today);
                day_repository.complete_week(date)?
            };
            let week = week
                .into_iter()
                .filter_map(|day| task_repository.day_with_tasks(day).ok());
            for day_with_tasks in week {
                term.day_with_tasks(&day_with_tasks);
            }
        }
        ("get", _) => {
            let Ok(today) = day_repository.today() else {
                term.error("could not get todays day!");
                term.end();
                return Ok(());
            };
            if let Ok(task) = task_repository.current(today) {
                term.task(&task);
            }
        }
        ("is_active", _) => {
            let Ok(today) = day_repository.today() else {
                term.error("could not get todays day!");
                term.end();
                exit(1);
            };
            let Ok(task) = task_repository.current(today) else {
                term.error("no task is currently active");
                exit(1);
            };
            term.task(&task);
        }
        (command, _) => term.error(eyre!("the command {} is not implemented.", command)),
    }

    term.flush();
    term.end();
    Ok(())
}
