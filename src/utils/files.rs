use std::{
    env,
    fs::{self},
    os::unix::fs::PermissionsExt,
};

const KEY: &str = "PATH";

pub fn find_exe_in_env(cmd: &str) -> Option<String> {
    let paths = env::var(KEY).ok()?;

    for path in env::split_paths(&paths) {
        let cmd_path = format!("{}/{}", path.display(), cmd);

        if let Ok(metadata) = fs::metadata(&cmd_path)
            && metadata.is_file()
            && metadata.permissions().mode() & 0o111 != 0
        {
            return Some(cmd_path);
        }
    }
    None
}
