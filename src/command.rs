use crate::{
    commands::{echo::Echo, exit::Exit, notfound::NotFound},
    execute::Execute,
};

pub enum Command {
    Exit,
    NotFound(String),
    Echo(String),
}
impl Command {
    pub fn match_cmd(cmd: &[&str]) -> Command {
        match cmd[0] {
            "exit" => Command::Exit,
            "echo" => Command::Echo(cmd[1..].join(" ")),
            _ => Command::NotFound(cmd[0].to_string()),
        }
    }
}

impl Execute for Command {
    fn execute(&self) {
        match self {
            Command::Exit => Exit::execute(),
            Command::NotFound(cmd) => {
                NotFound::new(cmd).execute();
            }
            Command::Echo(cmd) => Echo::new(cmd).execute(),
        }
    }
}
