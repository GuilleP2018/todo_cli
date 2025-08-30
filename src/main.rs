use clap::{Parser, Subcommand};
use colored::*;
use serde::{Serialize, Deserialize};
use std::{fs, process};

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

#[derive(Parser)]
#[command(name = "todo", version = "1.0", about = "A simple CLI To-Do app in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: Vec<String>},
    List,
    Done {index: usize},
    Remove {index: usize},
}


fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let desc = description.join(" ");
            tasks.push(Task {
                description: desc.clone(),
                done: false,
            });
            save_tasks(&tasks);
            println!("{} {}", "✔️ Added task:".green().bold(), desc);
        }

        Commands::List => {
            if tasks.is_empty() {
                println!("{}", "🎉 No tasks yet!".yellow().bold());
            } else {
                println!("{}", "📋 Your tasks:".blue().bold());
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done {
                        "[x]".green()
                    } else {
                        "[ ]".red()
                    };
                    println!("{} {} {}", i.to_string().cyan(), status, task.description);
                }
            }
        }

        Commands::Done { index } => {
            if index < tasks.len() {
                tasks[index].done = true;
                save_tasks(&tasks);
                println!(
                    "{} {}",
                    "✔️ Marked as done:".green().bold(),
                    tasks[index].description
                );
            } else {
                eprintln!("{}", "❌ Invalid index".red().bold());
                process::exit(1);
            }
        }

        Commands::Remove { index } => {
            if index < tasks.len() {
                let removed = tasks.remove(index);
                save_tasks(&tasks);
                println!(
                    "{} {}",
                    "🗑️ Removed task:".yellow().bold(),
                    removed.description
                );
            } else {
                eprintln!("{}", "❌ Invalid index".red().bold());
                process::exit(1);
            }
        }
    }
}