mod command;
mod commands;
mod utils;

use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use log::info;

use crate::command::Command;

fn main() {
    setup_logger().unwrap();

    info!("Shell started");

    let mut input = String::new();
    loop {
        print!("$ ");

        if let Err(err) = io::stdout().flush() {
            eprintln!("error: {}", err)
        }

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read command");

        if input.is_empty() {
            continue;
        }

        info!("Raw input: {}", &input);

        let parsed_cmd = Command::new(&input);

        parsed_cmd.execute();
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("output.log")?,
        )
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
