mod command;
mod commands;
mod execute;
mod utils;

use std::io::{self, Write};

use crate::{command::Command, execute::Execute};

fn main() {
    let mut input = String::new();
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();
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
