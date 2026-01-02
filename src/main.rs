#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut cmd = String::new();

    loop {
        cmd.clear();
        io::stdin().read_line(&mut cmd).unwrap();
        cmd = cmd.trim_end().trim_start().to_string();
        println!("{cmd}: command not found")
    }
}
