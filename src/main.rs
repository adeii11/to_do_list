mod cli;
mod tasks;

use cli::build_cli;
use tasks::TaskList;

fn main() {
    let matches = build_cli().get_matches();

    let mut task_list = TaskList::load().unwrap_or_else(|_| TaskList::new());

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            if let Some(task) = sub_m.get_one::<String>("TASK") {
                task_list.add_task(task.to_string());
                println!("Added task: {}", task);
            }
        }
        Some(("remove", sub_m)) => {
            if let Some(id) = sub_m.get_one::<String>("ID") {
                if let Ok(id) = id.parse::<u32>() {
                    task_list.remove_task(id);
                    println!("Removed task with ID: {}", id);
                } else {
                    eprintln!("Invalid ID");
                }
            }
        }
        Some(("list", _)) => {
            task_list.list_tasks();
        }
        _ => eprintln!("Invalid command"),
    }
}
