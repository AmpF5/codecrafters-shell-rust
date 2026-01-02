use crate::{
    commands::{exit::Exit, notfound::NotFound},
    execute::Execute,
};

pub enum Command {
    Exit,
    NotFound(String),
}
impl Command {
    pub fn match_cmd(cmd: &str) -> Command {
        match cmd {
            "exit" => Command::Exit,
            _ => Command::NotFound(cmd.to_string()),
        }
    }
}

impl Execute for Command {
    fn execute(&self) {
        match self {
            Command::Exit => Exit::execute(),
            Command::NotFound(_) => {
                NotFound::new(self).execute();
            }
        }
    }
}
