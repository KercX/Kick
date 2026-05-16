mod commands;
mod utils;

use colored::*;
use std::io::{self, Write};

fn main() {
    utils::banner::show_banner();

    loop {
        print!("{}", "Kick> ".bright_green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        match cmd {
            "help" => commands::help::run(),
            "clear" => commands::clear::run(),
            "cmd" => commands::cmd::run(),
            "powershell" => commands::powershell::run(),
            "files" => commands::files::run(),
            "exit" => {
                println!("Exiting Kick...");
                break;
            }
            _ => {
                if cmd.starts_with("run ") {
                    commands::run::run(cmd);
                } else {
                    println!("Unknown command.");
                }
            }
        }
    }
}
