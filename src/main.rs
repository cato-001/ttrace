use clap::{Arg, ArgAction, Command};
use config::Config;
use database::open_database_connection;
use day::DayRepository;

use crate::task::TaskRepository;

mod commands;
mod config;
mod database;
mod day;
mod task;

fn main() -> eyre::Result<()> {
    let cli = Command::new("ttrack")
        .subcommands([
            Command::new("start")
                .args([
                    Arg::new("description")
                        .long("description")
                        .short('d')
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
            Command::new("list")
                .args([
                    Arg::new("date")
                        .long("date")
                        .short('d')
                        .conflicts_with_all(["today", "week"])
                        .help("the date of the day to list the times of"),
                    Arg::new("today")
                        .long("today")
                        .short('t')
                        .action(ArgAction::SetTrue)
                        .default_value("true")
                        .conflicts_with_all(["date", "week"])
                        .help("list todays tasks"),
                    Arg::new("week")
                        .long("week")
                        .short('w')
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["date", "today"])
                        .help("list the tasks from the current week"),
                ])
                .about("list the tracked times"),
        ])
        .about("track the time you spend on projects or other tasks")
        .subcommand_required(true)
        .get_matches();

    let config = Config::load();
    let connection = open_database_connection(&config)?;

    let day_repository = DayRepository::new(connection.clone())?;
    let task_repository = TaskRepository::new(connection)?;

    match cli.subcommand().unwrap() {
        ("start", command) => {
            let description: &String = command.get_one("description").unwrap();
            let today = day_repository.today()?;
            let task = task_repository.start(&today, description.as_str())?;
            println!("{}", task);
        }
        ("stop", _) => {
            let today = day_repository.today()?;
            let task_id = task_repository.stop(&today)?;
            let task = task_repository.task(task_id)?;
            println!("{}", task)
        }
        ("list", command) => {
            if let Some(date) = command.get_one("date") {
                let day = day_repository.get_from_date(&date)?;
                let tasks = task_repository.tasks_for_day(&day)?;

                for task in tasks {
                    println!("{}", task)
                }

                return Ok(());
            }
            if let Some(true) = command.get_one("week") {
                return Ok(());
            }
            if let Some(true) = command.get_one("today") {
                let today = day_repository.today()?;
                let tasks = task_repository.tasks_for_day(&today)?;

                for task in tasks {
                    println!("{}", task)
                }

                return Ok(());
            }
        }
        (command, _) => {
            eprintln!("ERROR \"the command {} is not implemented.\"", command)
        }
    }

    Ok(())
}
