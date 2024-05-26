# TTrace

A cli application to track time for work.

## Usage

Start a task:

    // the currently running task is stopped automatically
    ttrace start "task description ..."

Stop a task:

    ttrace stop

Rename a task:

    ttrace rename "another task description ..."

Restart a task:

    // the end time of the previous task is also adjusted

    // restart at 10:30
    ttrace restart 1030

    // restart relative to current start time
    ttrace restart -20
    ttrace restart +20

List the tasks:

    // currently running task
    ttrace get
    
    // other
    ttrace today
    ttrace yesterday

    ttrace day
    ttrace day -2
    ttrace day YY-MM-DD

    ttrace week
    ttrace week -2

## Installation

You can install the cli application using cargo:

    cargo install --locked ttrace

