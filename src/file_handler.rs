use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::tasks::Task;

const FILE_PATH: &str = "tasks.txt";

pub fn get_next_id() -> usize {
    if Path::new(FILE_PATH).exists() {
        let file = OpenOptions::new()
            .read(true)
            .open(FILE_PATH)
            .expect("Unable to open file");

        let reader = BufReader::new(file);

        reader.lines().count() + 1
    } else {
        1
    }
}

pub fn write_task_to_file(task: &Task) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)
        .expect("Unable to open file");

    if let Err(e) = writeln!(file, "{}:{}", task.id, task.description) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn list_tasks_from_file() {
    if Path::new(FILE_PATH).exists() {
        let file = OpenOptions::new()
            .read(true)
            .open(FILE_PATH)
            .expect("Unable to open file");

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(task) => println!("{}", task),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        println!("No tasks found.");
    }
}

pub fn remove_task_from_file(id: usize) {
    if Path::new(FILE_PATH).exists() {
        let file = OpenOptions::new()
            .read(true)
            .open(FILE_PATH)
            .expect("Unable to open file");

        let reader = BufReader::new(file);

        let tasks: Vec<String> = reader.lines()
            .filter_map(|line| line.ok())
            .filter(|task| !task.starts_with(&format!("{}:", id)))
            .collect();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FILE_PATH)
            .expect("Unable to open file");

        for task in tasks {
            if let Err(e) = writeln!(file, "{}", task) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}
