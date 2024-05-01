use crate::task::DayWithTasks;

pub fn unstyled_day_with_tasks(day_with_tasks: &DayWithTasks) {
    println!("{}", day_with_tasks.day());
    for task in day_with_tasks.tasks() {
        println!("{}", task);
    }
}
