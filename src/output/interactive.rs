use colored::Colorize;
use term_painter::{Color, ToStyle};

use crate::{
    day::Day,
    output::interactive::{date_fmt::DateFmt, delta_fmt::DeltaFmt, time_fmt::TimeFmt},
    task::{DayWithTasks, Task, TaskGroup},
};

mod date_fmt;
mod delta_fmt;
mod time_fmt;

pub fn interactive_day_with_tasks(day_with_tasks: &DayWithTasks) {
    println!(
        "{} {} {}",
        "Day".green().bold(),
        DateFmt::new(day_with_tasks.day().date()),
        format!("({})", DeltaFmt::new(day_with_tasks.delta())).bright_black()
    );
    if day_with_tasks.is_empty() {
        println!(
            " {} {}",
            "->".bright_red(),
            "no tasks are recorded".bright_black()
        )
    }
    println!();
    for group in day_with_tasks.task_groups() {
        interactive_output_task_group(&group);
        println!()
    }
}

pub fn interactive_task(task: &Task) {
    println!("{} {}", "Task".blue().bold(), task.description());
    output_task_body(task);
}

pub fn interactive_output_task_group(group: &TaskGroup) {
    print!("{} {} ", "Task".blue().bold(), group.description(),);
    Color::White.with(|| println!("({})", DeltaFmt::new(group.delta())));

    for task in group.tasks() {
        output_task_body(task);
    }
}

fn output_task_body(task: &Task) {
    let arrow = if task.is_active() {
        "->".bright_green().bold()
    } else {
        "->".red()
    };
    print!(" {} {} ", arrow, DeltaFmt::option(task.delta()));
    Color::BrightBlack.with(|| {
        println!(
            "({} - {})",
            TimeFmt::new(task.start()),
            TimeFmt::option(task.end()),
        )
    })
}
