use std::io::{stdout, IsTerminal};

use colored::Colorize;

use crate::{
    output::{interactive::interactive_output_task_group, unstyled::unstyled_day_with_tasks},
    task::{DayWithTasks, Task},
};

use self::interactive::{interactive_day_with_tasks, interactive_task};

mod interactive;
mod unstyled;

fn out_interactive() -> bool {
    stdout().is_terminal()
}

pub fn output_task(task: &Task) {
    if !out_interactive() {
        println!("{}", task);
        return;
    }

    println!();
    interactive_task(task);
    println!();
}

pub fn output_day_with_tasks(day_with_tasks: DayWithTasks) {
    let day = day_with_tasks.day();
    if !out_interactive() {
        unstyled_day_with_tasks(&day_with_tasks);
        return;
    }

    println!();
    interactive_day_with_tasks(&day_with_tasks);
}

pub fn output_week(week: impl Iterator<Item = DayWithTasks>) {
    if !out_interactive() {
        for day_with_tasks in week {
            unstyled_day_with_tasks(&day_with_tasks);
        }
        return;
    }

    println!("");
    for day_with_tasks in week {
        interactive_day_with_tasks(&day_with_tasks);
    }
}

pub fn error_day_not_found() {
    if !out_interactive() {
        eprintln!("ERROR \"day could not be found\"");
        return;
    }

    println!();
    println!("{} could not find day", "Error".red().bold());
    println!();
}

pub fn error_no_task_started() {
    if !out_interactive() {
        eprintln!("ERROR \"no tasks are started\"");
        return;
    }

    println!();
    println!("{} no tasks are started", "Error".red().bold());
    println!();
}
