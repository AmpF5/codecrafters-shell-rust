use crate::commands::{self};

pub const COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

#[derive(Clone)]
pub enum Command {
    Exit,
    NotFound { cmd: String },
    Echo { args: String },
    Type { args: String },
    Exec { cmd: String, args: String },
    Pwd,
    Cd { args: String },
}

impl Command {
    pub fn new(input: &str) -> Command {
        let (cmd, args) = crate::utils::string::get_cmd_and_args(input);
        println!(
            "parsed {:?}",
            crate::utils::string::get_formatted_input(input)
        );

        if COMMANDS.contains(&cmd.as_str()) {
            match cmd.as_str() {
                "exit" => Command::Exit,
                "echo" => Command::Echo {
                    args: args.unwrap_or_default(),
                },
                "type" => Command::Type {
                    args: args.unwrap_or_default(),
                },
                "pwd" => Command::Pwd,
                "cd" => Command::Cd {
                    args: args.unwrap_or("~".to_owned()),
                },
                _ => Command::NotFound { cmd },
            }
        } else {
            match crate::utils::files::find_exe_in_env(&cmd) {
                Some(_) => Command::Exec {
                    cmd,
                    args: args.unwrap_or_default(),
                },
                None => Command::NotFound { cmd },
            }
        }
    }

    pub fn execute(&self) {
        match self {
            Command::Exit => commands::exit::execute(),
            Command::NotFound { cmd } => commands::notfound::execute(cmd),
            Command::Exec { cmd, args } => commands::exec::execute(cmd, args),
            Command::Type { args } => commands::r#type::execute(args),
            Command::Echo { args } => commands::echo::execute(args),
            Command::Pwd => commands::pwd::execute(),
            Command::Cd { args } => commands::cd::execute(args),
        }
    }
}
