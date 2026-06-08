use std::{collections::HashSet, fs::File, hash::Hash};

use colored::Colorize;

use crate::task::Task;

mod task;

const EMPTY_BOX: &str = "\u{2610}"; // ☐ (Ballot Box)
const CHECK_MARK: &str = "\u{2713}"; // ✓ (Check Mark)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut path = dirs::data_dir().unwrap();
    path.push("/todac/tasks.json");

    let file = File::open(path)?;
    let tasks: Vec<task::Task> = serde_json::from_reader(file)?;

    // println!("{} {} !", "it".green(), "works".blue().bold());
    // println!("{}", EMPTY_BOX);

    match args.len() {
        1 => output_list(&tasks),
        2 => println!("more args"),
        _ => {
            let first_cmd = args[1].as_str();
            let rest = &args[2..];

            if !has_duplicates(rest){
                println!("Duplicate input detected");
            }

            match first_cmd {
                "add" => {
                    for task in rest {
                        add_task(task.to_string());
                    }
                },
                _ => println!("invalid command")
            }
        }
    }

    output_list(&tasks);
    Ok(())
}

fn add_task(title: String){
    let task = Task {
        title,
        done: false
    };
}

fn output_list(tasks: &[task::Task]){
    if tasks.is_empty() {
        println!("Tasks empty");
        return;
    }
    let mut incrementer = 1;
    for task in tasks {
        match task.done {
            true => println!("{}. {} {}", incrementer, CHECK_MARK.green(), task.title.as_str().on_cyan()),
            false => println!("{}. {} {}", incrementer, EMPTY_BOX.blue(), task.title),
        }
        incrementer += 1;
    }
}

fn has_duplicates<I>(iter: I) -> bool
where 
    I: IntoIterator,
    I::Item: Eq + std::hash::Hash,
{
    let mut seen = HashSet::new();
    for item in iter {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}
