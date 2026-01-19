use std::env;

pub struct Cd {}

impl Cd {
    pub fn execute(path: &str) {
        let path_type = path.chars().nth(0).expect("cannot get path");
        let new_path = match path_type {
            '/' => {
                if crate::utils::files::is_dir_exists(path) {
                    path
                } else {
                    println!("cd: {path}: No such file or directory!");
                    return;
                }
            }
            _ => todo!(),
        };

        env::set_current_dir(new_path).expect("cannot change path to {path}");
    }
}
