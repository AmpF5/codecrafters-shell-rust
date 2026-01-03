pub struct NotFound {}

impl NotFound {
    pub fn execute(command: &str) {
        println!("{}: command not found", command)
    }
}
