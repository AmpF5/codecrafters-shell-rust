use crate::{command::COMMANDS, utils::files::find_exe_in_env};

pub struct Type {}

impl Type {
    pub fn execute(values: &[&str]) {
        for &cmd in values.iter() {
            if COMMANDS.contains(&cmd) {
                println!("{} is a shell builtin", cmd);
            } else if let Some(path) = find_exe_in_env(cmd) {
                println!("{cmd} is {path}")
            } else {
                println!("{}: not found", cmd);
            }
        }
    }
}
