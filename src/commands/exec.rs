use std::process::Command;

pub fn execute(cmd: &str, args: &[&str]) {
    let mut command = Command::new(cmd);
    command.args(args);
    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            print!("{}", stdout);
        }
        Err(err) => eprintln!("error: {}", err),
    }
}
