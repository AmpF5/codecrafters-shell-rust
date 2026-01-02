use std::process;

pub struct Exit {}

impl Exit {
    pub fn execute() {
        process::exit(0);
    }
}
