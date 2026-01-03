use std::{env, fs, os::unix::fs::PermissionsExt};

use crate::command::COMMANDS;

pub struct Type {}

impl Type {
    pub fn execute(values: &[&str]) {
        let key = "PATH";
        for &command in values.iter() {
            if COMMANDS.contains(&command) {
                println!("{} is a shell builtin", command);
            } else if let Ok(paths) = env::var(key) {
                for path in env::split_paths(&paths) {
                    let path_to_command = format!("{}/{}", path.display(), command);
                    match fs::metadata(&path_to_command) {
                        Ok(file) => {
                            if file.is_file() && file.permissions().mode() & 0o111 != 0 {
                                println!("{command} is {path_to_command}");
                                return;
                            }
                        }
                        Err(_) => continue,
                    }
                }
                println!("{}: not found", command);
            } else {
                println!("{}: not found", command);
            }
        }
    }
}
