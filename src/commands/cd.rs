use std::env;

pub fn execute(path: &str) {
    let Some(path_type) = path.chars().next() else {
        eprintln!("cannot get path");
        return;
    };

    let new_path = match path_type {
        '/' => {
            if crate::utils::files::is_dir_exists(path) {
                path
            } else {
                eprintln!("cd: {path}: No such file or directory");
                return;
            }
        }
        _ => todo!(),
    };

    env::set_current_dir(new_path).expect("cannot change path to {path}");
}
