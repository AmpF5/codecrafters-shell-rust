use crate::{command::Command, execute::Execute};

pub struct NotFound {
    cmd: String,
}

impl NotFound {
    pub fn new(cmd: &Command) -> NotFound {
        match cmd {
            Command::NotFound(cmd) => NotFound {
                cmd: cmd.to_string(),
            },
            Command::Exit => panic!(),
        }
    }
}

impl Execute for NotFound {
    fn execute(&self) {
        println!("{}: command not found", self.cmd)
    }
}
