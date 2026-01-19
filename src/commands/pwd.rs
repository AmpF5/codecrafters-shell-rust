use std::env;

pub struct Pwd {}

impl Pwd {
    pub fn execute() {
        println!("{}", env::current_dir().unwrap_or_default().display());
    }
}
