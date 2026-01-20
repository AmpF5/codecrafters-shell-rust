use crate::commands::{self};

pub const COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

#[derive(Clone)]
pub enum Command<'a> {
    Exit,
    NotFound { command: &'a str },
    Echo { value: &'a [&'a str] },
    Type { values: &'a [&'a str] },
    Exec { value: &'a str, args: &'a [&'a str] },
    Pwd,
    Cd { path: &'a str },
}

impl<'a> Command<'a> {
    pub fn new(cmd: &'a [&'a str]) -> Command<'a> {
        let input = cmd[0];

        if COMMANDS.contains(&input) {
            match input {
                "exit" => Command::Exit,
                "echo" => Command::Echo { value: &cmd[1..] },
                "type" => Command::Type { values: &cmd[1..] },
                "pwd" => Command::Pwd,
                "cd" => {
                    if cmd.len() > 1 {
                        Command::Cd { path: cmd[1] }
                    } else {
                        Command::Cd { path: "~" }
                    }
                }
                _ => Command::NotFound { command: cmd[0] },
            }
        } else {
            match crate::utils::files::find_exe_in_env(input) {
                Some(_) => Command::Exec {
                    value: input,
                    args: &cmd[1..],
                },
                None => Command::NotFound { command: cmd[0] },
            }
        }
    }
}

impl<'a> Command<'a> {
    pub fn execute(&self) {
        match self {
            Command::Exit => commands::exit::execute(),
            Command::NotFound { command } => commands::notfound::execute(command),
            Command::Exec { value, args } => commands::exec::execute(value, args),
            Command::Type { values } => commands::r#type::execute(values),
            Command::Echo { value } => commands::echo::execute(value),
            Command::Pwd => commands::pwd::execute(),
            Command::Cd { path: value } => commands::cd::execute(value),
        }
    }
}
