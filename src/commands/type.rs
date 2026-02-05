use crate::{command::COMMANDS, utils::files::find_exe_in_env};

pub fn execute(values: &str) {
    for cmd in values.split_whitespace() {
        if COMMANDS.contains(&cmd) {
            println!("{} is a shell builtin", cmd);
        } else if let Some(path) = find_exe_in_env(cmd) {
            println!("{} is {}", cmd, path.display())
        } else {
            println!("{}: not found", cmd);
        }
    }
}
