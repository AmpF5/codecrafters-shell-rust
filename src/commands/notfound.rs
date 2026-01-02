use crate::{command::Command, execute::Execute};

pub struct NotFound {
    cmd: String,
}

impl NotFound {
    pub fn new(cmd: &String) -> NotFound {
        NotFound { cmd: cmd.clone() }
    }
}

impl Execute for NotFound {
    fn execute(&self) {
        println!("{}: command not found", self.cmd)
    }
}
