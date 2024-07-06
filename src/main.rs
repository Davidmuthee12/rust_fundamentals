mod task;
pub use task::task::Task;

use std::io;
use chrono::prelude::*;

fn main() {
    println!("Welcome to Rusty Task Manager!\n");
    let tasks: Vec<Task> = Vec::new();

    loop {
        println!("commands:");
        println!("- add <title> <description> <due_date>");
        println!("- view");
        println!("- complete <task_index>");
        println!("- filter <completed | upcoming>\n");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts[0] {
            "filter" => {
                if parts.len() == 2 {
                    match parts[1] {
                        "completed" => {
                            for (index, task) in tasks.iter().enumerate().filter(|t| t.1.completed) {
                                println!("{}. {} [Complete]", index + 1, task.title);
                            }
                        }
                        "upcoming" => {
                            let today = Local::now().naive_local().date();
                            for (index, task) in tasks.iter().enumerate().filter(|t| t.1.due_date >= today) {
                                let status = if task.completed { "[Complete]" } else { "[Incomplete]" };
                                println!("{}. {} {}", index + 1, task.title, status);
                            }
                        }
                        _ => println!("invalid filter option."),
                    }
                } else {
                    println!("invalid input. Use: filter <completed | upcoming>");
                }
            }
            _ => println!("Invalid Command."),
        }
    }
}
