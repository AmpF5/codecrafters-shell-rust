use std::env;

pub fn execute() {
    println!("{}", env::current_dir().unwrap_or_default().display());
}
