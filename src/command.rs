use crate::{
    commands::{echo::Echo, exec::Exec, exit::Exit, notfound::NotFound, r#type::Type},
    execute::Execute,
};

pub const COMMANDS: [&str; 3] = ["echo", "exit", "type"];

#[derive(Clone)]
pub enum Command<'a> {
    Exit,
    NotFound { command: &'a str },
    Echo { value: String },
    Type { values: &'a [&'a str] },
    Exec { value: &'a str, args: &'a [&'a str] },
}

impl<'a> Command<'a> {
    pub fn new(cmd: &'a [&'a str]) -> Command<'a> {
        let input = cmd[0];

        if COMMANDS.contains(&input) {
            match input {
                "exit" => Command::Exit,
                "echo" => Command::Echo {
                    value: cmd[1..].join(" "),
                },
                "type" => Command::Type { values: &cmd[1..] },
                _ => Command::NotFound { command: cmd[0] },
            }
        } else {
            match crate::utils::files::find_exe_in_env(input) {
                Some(_) => Command::Exec {
                    value: input,
                    args: &cmd[0..],
                },
                None => Command::NotFound { command: cmd[0] },
            }
        }
    }
}

impl<'a> Execute for Command<'a> {
    fn execute(&self) {
        match self {
            Command::Exit => Exit::execute(),
            Command::NotFound { command } => NotFound::execute(command),
            Command::Exec { value, args } => Exec::execute(value, args),
            Command::Type { values } => Type::execute(values),
            Command::Echo { value } => Echo::execute(value),
        }
    }
}
