use std::{
    env,
    fs::{self},
    os::unix::fs::PermissionsExt,
};

const KEY: &str = "PATH";

pub fn find_exe_in_env(cmd: &str) -> Option<String> {
    let path = find_path_in_env(cmd)?;
    let metadata = fs::metadata(&path).ok()?;
    if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
        Some(path)
    } else {
        None
    }
}

fn find_path_in_env(cmd: &str) -> Option<String> {
    let paths = env::var(KEY).ok()?;

    for path in env::split_paths(&paths) {
        let cmd_path = format!("{}/{}", path.display(), cmd);

        if fs::metadata(&cmd_path).is_ok() {
            return Some(cmd_path);
        }
    }
    None
}
