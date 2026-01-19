use std::process::Command;

pub struct Exec {}

impl Exec {
    pub fn execute(cmd: &str, args: &[&str]) {
        let mut command = Command::new(cmd);
        command.args(args);
        let output = command.output().expect("Faile to execute program");
        let stdout = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout);
    }
}
