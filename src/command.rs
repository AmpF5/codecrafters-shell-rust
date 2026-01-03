use crate::{
    commands::{echo::Echo, exit::Exit, notfound::NotFound, r#type::Type},
    execute::Execute,
};

pub const COMMANDS: [&str; 3] = ["echo", "exit", "type"];

#[derive(Clone)]
pub enum Command<'a> {
    Exit,
    NotFound { command: &'a str },
    Echo { value: String },
    Type { values: Vec<&'a str> },
}

impl<'a> Command<'a> {
    pub fn new(cmd: &'a [&'a str]) -> Command<'a> {
        match cmd[0] {
            "exit" => Command::Exit,
            "echo" => Command::Echo {
                value: cmd[1..].join(" "),
            },
            "type" => Command::Type {
                values: cmd.iter().skip(1).copied().collect(),
            },
            _ => Command::NotFound { command: cmd[0] },
        }
    }
}

impl<'a> Execute for Command<'a> {
    fn execute(&self) {
        match self {
            Command::Exit => Exit::execute(),
            Command::NotFound { command } => NotFound::execute(command),
            Command::Echo { value } => Echo::execute(value),
            Command::Type { values } => Type::execute(values),
        }
    }
}
