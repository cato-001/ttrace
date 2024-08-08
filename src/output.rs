use core::task;
use std::{
    fmt::Display,
    io::{stdout, IsTerminal},
};

use serde::Serialize;
use termfmt::{
    chrono::{DateFmt, DeltaFmt, TimeFmt},
    termarrow, termarrow_fg, termerr, termh1, termh2, termprefix1, termprefix2, BundleFmt, Fg,
    TermFmt, TermStyle,
};

use crate::{
    config::Config,
    day::Day,
    task::{DayWithTasks, Task, TaskGroup},
};

#[derive(Default, Serialize)]
pub struct DataBundle {
    config: Config,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    tasks: Vec<Task<Day>>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    day_with_tasks: Vec<DayWithTasks>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    info: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    error: Vec<String>,
}

pub trait OutputFmt {
    fn error(&mut self, value: impl Display);
    fn day_with_tasks(&mut self, value: &DayWithTasks);
    fn task(&mut self, task: &Task<Day>);
    fn end(&mut self);
}

impl OutputFmt for TermFmt<DataBundle> {
    fn error(&mut self, value: impl Display) {
        self.bundle(|bundle| bundle.error.push(format!("{}", value)));
        if self.is_plain() {
            println!("{}", value);
        }
        if self.is_interactive() {
            termerr(value);
        }
    }

    fn day_with_tasks(&mut self, value: &DayWithTasks) {
        self.bundle(|bundle| bundle.day_with_tasks.push(value.clone()));
        if self.is_plain() {
            println!("{}", value.day());
            for task in value.tasks() {
                println!("{}", task);
            }
        }
        if self.is_interactive() {
            termprefix1(
                "Day",
                format_args!(
                    "{} {}",
                    DateFmt::new(value.day().date()),
                    format_args!("({})", DeltaFmt::new(value.delta())).fg_bright_black()
                ),
            );
            if value.is_empty() {
                termarrow("no tasks recorded!".fg_bright_black());
            }
            for group in value.task_groups() {
                termprefix2(
                    "Task",
                    format_args!(
                        "{} {}",
                        group.description(),
                        format_args!("({})", DeltaFmt::new(group.delta())).fg_bright_black()
                    ),
                );
                for task in group.tasks() {
                    term_task_body(task);
                }
            }
        }
    }

    fn task(&mut self, value: &Task<Day>) {
        self.bundle(|bundle| bundle.tasks.push(value.clone()));
        self.plain(value);
        if self.is_interactive() {
            termprefix2("Task", value.description());
            term_task_body(&value);
        }
    }

    fn end(&mut self) {
        if self.is_interactive() {
            println!();
        }
    }
}

impl BundleFmt for DataBundle {
    type Config = Config;

    fn new(config: Self::Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    fn clear(&mut self) {
        self.tasks.clear();
        self.day_with_tasks.clear();
    }
}

fn term_task_body(task: &Task<Day>) {
    let color = if task.is_active() { Fg::Green } else { Fg::Red };
    termarrow_fg(
        color,
        format_args!(
            "{} {}",
            DeltaFmt::new(task.delta()),
            format_args!(
                "({} - {})",
                TimeFmt::new(task.start()),
                TimeFmt::option(task.end())
            )
            .fg_bright_black()
        ),
    );
}
