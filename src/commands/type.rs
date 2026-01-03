use crate::command::COMMANDS;

pub struct Type {}

impl Type {
    pub fn execute(values: &[&str]) {
        for &command in values.iter() {
            if COMMANDS.contains(&command) {
                println!("{} is a shell builtin", command)
            } else {
                println!("{}: not found", command)
            }
        }
    }
}
