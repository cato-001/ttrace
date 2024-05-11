#![allow(unused)]

use chrono::{Days, Local};
use clap::{Arg, ArgAction, Command};
use config::Config;
use database::open_database_connection;
use day::DayRepository;
use termfmt::{TermFmtExt, TermFmtsExt};

use crate::task::TaskRepository;

use self::output::{DataBundle, OutputFmt};

mod commands;
mod config;
mod database;
mod day;
mod output;
mod task;

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
            Command::new("week").about("list the task of this week"),
        ])
        .about("track the time you spend on projects or other tasks")
        .subcommand_required(true)
        .termfmts()
        .get_matches();

    let config = Config::load()?;
    let connection = open_database_connection(&config)?;

    let mut term = cli.termfmt(DataBundle::default());

    let day_repository = DayRepository::new(connection.clone())?;
    let task_repository = TaskRepository::new(connection)?;

    match cli.subcommand().unwrap() {
        ("start", command) => {
            let description: &String = command.get_one("description").unwrap();
            let today = day_repository.today()?;
            let task = task_repository.start(&today, description.as_str())?;
            term.task(task);
        }
        ("stop", _) => {
            let today = day_repository.today()?;
            let Ok(task_id) = task_repository.stop(&today) else {
                term.error("no task is started yet!");
                term.end();
                return Ok(());
            };
            let task = task_repository.task(task_id)?;
            term.task(task);
        }
        ("today", _) => {
            let Ok(today) = day_repository.today() else {
                term.error("could not find or create day!");
                term.end();
                return Ok(());
            };
            let tasks_for_day = task_repository.day_with_tasks(today)?;
            term.day_with_tasks(tasks_for_day);
        }
        ("yesterday", _) => {
            let Ok(yesterday) = day_repository.yesterday() else {
                term.error("could not find or create day!");
                term.end();
                return Ok(());
            };
            let tasks_for_day = task_repository.day_with_tasks(yesterday)?;
            term.day_with_tasks(tasks_for_day);
        }
        ("week", _) => {
            let week = day_repository.week_till_today()?;
            let week = week
                .into_iter()
                .filter_map(|day| task_repository.day_with_tasks(day).ok());
            for day_with_tasks in week {
                term.day_with_tasks(day_with_tasks);
            }
        }
        (command, _) => {
            eprintln!("ERROR \"the command {} is not implemented.\"", command)
        }
    }

    term.flush();
    term.end();
    Ok(())
}
