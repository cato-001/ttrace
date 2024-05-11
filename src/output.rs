use std::{
    fmt::Display,
    io::{stdout, IsTerminal},
};

use serde::Serialize;
use termfmt::{
    chrono::{DateFmt, DeltaFmt, TimeFmt},
    termarrow, termarrow_fg, termerr, termh1, termh2, termprefix1, termprefix2, BundleFmt, DataFmt,
    Fg, TermFmt, TermStyle,
};

use crate::{
    day::Day,
    task::{DayWithTasks, Task, TaskGroup},
};

pub enum OutputData {
    Error(String),
    Task(Task),
    DayWithTask(DayWithTasks),
    End,
}

#[derive(Default, Serialize)]
pub struct DataBundle {
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    tasks: Vec<Task>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    day_with_tasks: Vec<DayWithTasks>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    info: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    error: Vec<String>,
}

pub trait OutputFmt {
    fn error(&mut self, value: impl Display);
    fn day_with_tasks(&mut self, value: DayWithTasks);
    fn task(&mut self, task: Task);
    fn end(&mut self);
}

impl OutputFmt for TermFmt<OutputData, DataBundle> {
    fn error(&mut self, value: impl Display) {
        self.output(OutputData::Error(format!("{}", value)));
    }

    fn day_with_tasks(&mut self, value: DayWithTasks) {
        self.output(OutputData::DayWithTask(value));
    }

    fn task(&mut self, value: Task) {
        self.output(OutputData::Task(value));
    }

    fn end(&mut self) {
        self.output(OutputData::End);
    }
}

impl DataFmt for OutputData {
    fn plain(self) {
        match self {
            Self::Error(value) => eprintln!("{}", value),
            Self::Task(value) => println!("{}", value),
            Self::DayWithTask(value) => {
                println!("{}", value.day());
                for task in value.tasks() {
                    println!("{}", task);
                }
            }
            Self::End => {}
        }
    }

    fn interactive(self) {
        match self {
            Self::Error(value) => termerr(value),
            Self::Task(value) => {
                termprefix2("Task", value.description());
                term_task_body(&value);
            }
            Self::DayWithTask(value) => {
                termprefix1("Day", DateFmt::new(value.day().date()));
                if value.is_empty() {
                    termarrow("no tasks recorded!".fg_bright_black());
                }
                for group in value.task_groups() {
                    termprefix2("Task", group.description());
                    for task in group.tasks() {
                        term_task_body(task);
                    }
                }
            }
            Self::End => println!(),
        }
    }
}

impl BundleFmt for DataBundle {
    type Data = OutputData;

    fn push(&mut self, value: Self::Data) {
        match value {
            OutputData::Error(value) => self.error.push(value),
            OutputData::Task(value) => self.tasks.push(value),
            OutputData::DayWithTask(value) => self.day_with_tasks.push(value),
            OutputData::End => {}
        }
    }

    fn clear(&mut self) {
        self.tasks.clear();
        self.day_with_tasks.clear();
    }
}

fn term_task_body(task: &Task) {
    let color = if task.is_active() { Fg::Green } else { Fg::Red };
    termarrow_fg(
        color,
        format_args!(
            "{} {}",
            DeltaFmt::option(task.delta()),
            format_args!(
                "({} - {})",
                TimeFmt::new(task.start()),
                TimeFmt::option(task.end())
            )
            .fg_bright_black()
        ),
    );
}
