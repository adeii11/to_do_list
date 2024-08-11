use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task { id, description };
        self.tasks.push(task);
        self.save().expect("Failed to save tasks");
    }

    pub fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
        self.save().expect("Failed to save tasks");
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{}: {}", task.id, task.description);
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(&self).expect("Failed to serialize tasks");
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("tasks.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn load() -> io::Result<Self> {
        let data = fs::read_to_string("tasks.json")?;
        let task_list: TaskList = serde_json::from_str(&data).expect("Failed to deserialize tasks");
        Ok(task_list)
    }
}
