
use serde::{Serialize, Deserialize};
use std::{env, fs, process};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

const FILE: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE, data).expect("Unable to write file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: todo_cli [add <task> | list | done <index> | remove <index>]");
        process::exit(1);
    }

    let mut tasks = load_tasks();

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Please provide a task description.");
                process::exit(1);
            }
            let description = args[2..].join(" ");
            tasks.push(Task {description, done: false});
            save_tasks(&tasks);
            println!("Task Added!");
        }

        "list" => {
            if tasks.is_empty() {
                println!("No tasks yet");
            }else {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done {"[x]"} else {"[ ]"};
                    println!("{} {} {}", i, status, task.description);
                }
            }
        }

        "done" => {
            if args.len() < 3 {
                eprintln!("Please provide the index of the task to mark as done.");
                process::exit(1);
            }
            let index: usize = args[2].parse().expect("Index must be a number");
            if index < tasks.len(){
                tasks[index].done = true;
                save_tasks(&tasks);
                println!("Task Marked as Done");
            } else {
                eprintln!("Invalid Index");
            }
        }

        "remove" => {
            if args.len() < 3 {
                eprintln!("Please provide the index of the task to remove.");
                process::exit(1);
            }
            let index: usize = args[2].parse().expect("Index must be a number");
            if index < tasks.len() {
                tasks.remove(index);
                save_tasks(&tasks);
                println!("Task Removed");
            } else {
                eprintln!("Invalid Index");
            }
        }

        _=>{
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
