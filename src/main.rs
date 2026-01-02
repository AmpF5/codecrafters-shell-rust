mod command;
mod commands;
mod execute;

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

        let parsed_cmd = Command::match_cmd(input.trim());

        parsed_cmd.execute();
    }
}
