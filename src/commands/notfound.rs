use crate::execute::Execute;

pub struct NotFound {
    cmd: String,
}

impl NotFound {
    pub fn new(cmd: &str) -> NotFound {
        NotFound {
            cmd: cmd.to_string(),
        }
    }
}

impl Execute for NotFound {
    fn execute(&self) {
        println!("{}: command not found", self.cmd)
    }
}
