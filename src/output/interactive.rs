use colored::Colorize;
use term_painter::{Color, ToStyle};

use crate::{
    day::Day,
    output::interactive::{date_fmt::DateFmt, delta_fmt::DeltaFmt, time_fmt::TimeFmt},
    task::{Task, TaskGroup},
};

mod date_fmt;
mod delta_fmt;
mod time_fmt;

pub fn interactive_output_day(day: &Day) {
    println!("{} {}", "Day".green().bold(), DateFmt::new(day.date()));
}

pub fn interactive_output_task(task: &Task) {
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
        "->".magenta()
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
