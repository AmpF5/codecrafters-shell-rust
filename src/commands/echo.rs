use crate::execute::Execute;

pub struct Echo {
    cmd: String,
}

impl Echo {
    pub fn new(cmd: &str) -> Echo {
        Echo {
            cmd: cmd.to_string(),
        }
    }
}

impl Execute for Echo {
    fn execute(&self) {
        println!("{}", self.cmd)
    }
}
