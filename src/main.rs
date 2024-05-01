#![allow(unused)]

use chrono::{Days, Local};
use clap::{Arg, ArgAction, Command};
use config::Config;
use database::open_database_connection;
use day::DayRepository;
use output::{error_day_not_found, output_day_with_tasks, output_task, output_week};

use crate::task::TaskRepository;

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
        .get_matches();

    let config = Config::load()?;
    let connection = open_database_connection(&config)?;

    let day_repository = DayRepository::new(connection.clone())?;
    let task_repository = TaskRepository::new(connection)?;

    match cli.subcommand().unwrap() {
        ("start", command) => {
            let description: &String = command.get_one("description").unwrap();
            let today = day_repository.today()?;
            let task = task_repository.start(&today, description.as_str())?;
            output_task(&task);
        }
        ("stop", _) => {
            let today = day_repository.today()?;
            let task_id = task_repository.stop(&today)?;
            let task = task_repository.task(task_id)?;
            output_task(&task);
        }
        ("today", _) => {
            let Ok(today) = day_repository.today() else {
                error_day_not_found();
                return Ok(());
            };
            let tasks_for_day = task_repository.day_with_tasks(today)?;
            output_day_with_tasks(tasks_for_day);
        }
        ("yesterday", _) => {
            let Ok(yesterday) = day_repository.yesterday() else {
                error_day_not_found();
                return Ok(());
            };
            let tasks_for_day = task_repository.day_with_tasks(yesterday)?;
            output_day_with_tasks(tasks_for_day);
        }
        ("week", _) => {
            let week = day_repository.week_till_today()?;
            let week = week
                .into_iter()
                .filter_map(|day| task_repository.day_with_tasks(day).ok());
            output_week(week);
        }
        (command, _) => {
            eprintln!("ERROR \"the command {} is not implemented.\"", command)
        }
    }

    Ok(())
}
