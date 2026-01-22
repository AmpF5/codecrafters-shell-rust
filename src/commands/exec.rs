use std::process::Command;

pub fn execute(cmd: &str, args: &str) {
    let mut command = Command::new(cmd);
    command.args(crate::utils::string::get_formatted_input(args));
    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            print!("{}", stdout);
        }
        Err(err) => eprintln!("error: {}", err),
    }
}
