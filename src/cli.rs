use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("to_do_list")
        .about("A simple to-do list CLI")
        .arg(Arg::new("TASK").help("The task to add").required(false))
        .arg(Arg::new("ID").help("The ID of the task to remove").required(false))
        .subcommand(Command::new("add").about("Add a task").arg(Arg::new("TASK").required(true)))
        .subcommand(Command::new("remove").about("Remove a task").arg(Arg::new("ID").required(true)))
        .subcommand(Command::new("list").about("List all tasks"))
}
