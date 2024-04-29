use std::io::{stdout, IsTerminal};

use crate::{
    output::interactive::{interactive_output_day, interactive_output_task_group},
    task::{DayWithTasks, Task},
};

use self::interactive::interactive_output_task;

mod interactive;

fn out_interactive() -> bool {
    stdout().is_terminal()
}

pub fn output_task(task: &Task) {
    if !out_interactive() {
        println!("{}", task);
        return;
    }

    println!();
    interactive_output_task(task);
    println!();
}

pub fn output_day_with_tasks(day_with_tasks: DayWithTasks) {
    let day = day_with_tasks.day();
    if !out_interactive() {
        println!("{}", day);
        for task in day_with_tasks.tasks() {
            println!("{}", task);
        }
        return;
    }

    println!();
    interactive_output_day(day);
    println!();
    for group in day_with_tasks.task_groups() {
        interactive_output_task_group(&group);
        println!()
    }
}
