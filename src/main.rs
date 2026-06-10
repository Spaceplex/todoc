use std::{collections::HashSet, fs::File, io::{Seek, SeekFrom}};

use colored::Colorize;

use crate::task::Task;

mod task;

const EMPTY_BOX: &str = "\u{2610}"; // ☐ (Ballot Box)
const CHECK_MARK: &str = "\u{2713}"; // ✓ (Check Mark)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut path = dirs::data_dir().ok_or("Could not find data directory")?;
    path.push("todac");
    std::fs::create_dir_all(&path)?;
    path.push("tasks.json");

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;
    let mut tasks: Vec<task::Task> = if file.metadata()?.len() == 0 {
        vec![]
    } else {
        serde_json::from_reader(&file)?
    };

    match args.len() {
        1 => {
            output_list(&tasks);
            return Ok(());
        },
        2 => {
            println!("more args");
            return Ok(());
        }
        _ => {
            let first_cmd = args[1].as_str();
            let rest = &args[2..];

            if has_duplicates(rest){
                println!("Duplicate input detected");
            }

            match first_cmd {
                "add" => {
                    for task in rest {
                        add_task(task.to_string(), &mut tasks);
                    }
                },
                "done" => {
                    let r: Result<Vec<usize>, _> = rest.iter().map(|n| n.parse::<usize>()).collect();
                    let r = r.map_err(|_| "All inputs must be numbers for \"done\"")?;
                    if r.iter().any(|&n| n < 1 || n > tasks.len()) {
                        return Err(String::from("Out of bound number passed").into());
                    }
                    for n in r {
                        tasks[n - 1].done = true;
                    }
                }
                "delete" => {
                    let r: Result<Vec<usize>, _> = rest.iter().map(|n| n.parse::<usize>()).collect();
                    let r = r.map_err(|_| "All inputs must be numbers for \"done\"")?;
                    let mut s = r.clone();
                    s.sort_by(|a,b| b.cmp(a));
                    let s = s;
                    if s.iter().any(|&n| n < 1 || n > tasks.len()) {
                        return Err(String::from("Out of bound number passed").into());
                    }
                    for n in s {
                        tasks.remove(n - 1);
                    }
                }
                _ => {
                    println!("invalid command");
                    return Ok(());
                }
            }
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer_pretty(&file, &tasks)?;

    output_list(&tasks);
    Ok(())
}

fn add_task(title: String, tasks: &mut Vec<Task>){
    let task = Task {
        title,
        done: false
    };
    tasks.push(task);
}

fn output_list(tasks: &[task::Task]){
    if tasks.is_empty() {
        println!("Tasks empty");
        return;
    }
    tasks.iter().enumerate().for_each(|tsk| {
        match tsk.1.done {
            true => println!("{}. {} {}", tsk.0 + 1, CHECK_MARK.green(), tsk.1.title.as_str().truecolor(150, 150, 150)),
            false => println!("{}. {} {}", tsk.0 + 1, EMPTY_BOX.blue(), tsk.1.title),
        }
    });
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
