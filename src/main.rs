mod command;
mod commands;
mod utils;

use std::io::{self, Write};

use crate::command::Command;

fn main() {
    let mut input = String::new();
    loop {
        print!("$ ");

        if let Err(err) = io::stdout().flush() {
            eprintln!("error: {}", err)
        }

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read command");

        let cmd = input.split_whitespace().collect::<Vec<&str>>();
        if cmd.is_empty() {
            continue;
        }

        let parsed_cmd = Command::new(&cmd);

        parsed_cmd.execute();
    }
}
