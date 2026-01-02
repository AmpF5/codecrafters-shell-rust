#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read command");

        let cmd = input.trim();

        println!("{cmd}: command not found")
    }
}
