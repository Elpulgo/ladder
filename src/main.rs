extern crate clipboard_ext;
extern crate structopt;

use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;
use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};
use std::process;
use structopt::StructOpt;

static COMMAND_FILE_NAME: &str = "/commands.dat";

fn main() {
    let args = Cli::from_args();

    if !args.command_text.is_empty() && args.command_text.len() == 2 {
        persist_command(&args.command_text[0], &args.command_text[1]);
        return;
    }

    let stdin = io::stdin();
    let mut numeric_input: i32;

    let commands = read_commands();

    for command in commands.iter() {
        println!(
            "{}. {}",
            command.numeric_representation, command.description
        );
	println!("\t>>> {}\n", command.value);
    }

    for line in stdin.lock().lines() {
        let input = line.unwrap();

        numeric_input = match input.parse::<i32>() {
            Ok(numeric) => numeric,
            Err(_err) => {
                println!("Must specify a numeric alternative!");
                continue;
            }
        };

        if let Some(command) = commands
            .iter()
            .find(|&c| c.numeric_representation == numeric_input)
        {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(command.value.to_string()).unwrap();
            break;
        } else {
            println!("No ladder found for {}", numeric_input);
            continue;
        }
    }
}

pub fn read_commands() -> Vec<Command> {
    let file: Option<File> = match File::open(build_file_path()) {
        Ok(file) => Some(file),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Insufficient permissions to read command file. Try with admin rights.");
                process::exit(0);
            }
            ErrorKind::NotFound => None,
            _ => {
                eprintln!("Failed to open command file '{}'", e);
                process::exit(0);
            }
        },
    };

    match file {
        Some(file) => {
            let reader = BufReader::new(file);

            let commands: Vec<Command> = match serde_json::from_reader(reader) {
                Ok(commands) => commands,
                Err(e) => {
                    eprintln!("Failed to parse content '{}'", e);
                    process::exit(0);
                }
            };

            return commands;
        }
        None => {
            return Vec::<Command>::new();
        }
    }
}

pub fn persist_command(description: &String, value: &String) {
    let file: Option<File> = match File::open(build_file_path()) {
        Ok(file) => Some(file),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Insufficient permissions to read command file. Try with admin rights.");
                process::exit(0);
            }
            ErrorKind::NotFound => None,
            _ => {
                eprintln!("Failed to open command file '{}'", e);
                process::exit(0);
            }
        },
    };

    match file {
        Some(file) => {
            let reader = BufReader::new(file);

            let mut commands: Vec<Command> = match serde_json::from_reader(reader) {
                Ok(commands) => commands,
                Err(e) => {
                    eprintln!("Failed to parse content '{}'", e);
                    process::exit(0);
                }
            };

            commands.push(Command {
                numeric_representation: commands.len() as i32 + 1,
                description: description.to_string(),
                value: value.to_string(),
            });

            let new_file = match File::create(build_file_path()) {
                Ok(file) => file,
                Err(e) => match e.kind() {
                    ErrorKind::PermissionDenied => {
                        println!("Insufficient permissions to create command file. Try with admin rights.");
                        process::exit(0);
                    }
                    ErrorKind::NotFound => {
                        println!("Directory to save commands not found.");
                        process::exit(0);
                    }
                    _ => {
                        eprintln!("Failed to create command file '{}'", e);
                        process::exit(0);
                    }
                },
            };

            if serde_json::to_writer_pretty(&new_file, &commands).is_err() {
                eprintln!("Failed to persist command :(");
                process::exit(0);
            }

            println!("Successfully saved!");
        }
        None => {
            let new_file = match File::create(build_file_path()) {
                Ok(file) => file,
                Err(e) => match e.kind() {
                    ErrorKind::PermissionDenied => {
                        println!("Insufficient permissions to create command file. Try with admin rights.");
                        process::exit(0);
                    }
                    ErrorKind::NotFound => {
                        println!("Directory to save commands not found.");
                        process::exit(0);
                    }
                    _ => {
                        eprintln!("Failed to create command file '{}'", e);
                        process::exit(0);
                    }
                },
            };

            let mut commands = Vec::new();
            commands.push(Command {
                numeric_representation: 1,
                description: description.to_string(),
                value: value.to_string(),
            });

            if serde_json::to_writer_pretty(&new_file, &commands).is_err() {
                eprintln!("Failed to persist command :(");
                process::exit(0);
            }
        }
    }
}

fn build_file_path() -> String {
    let mut current_dir = current_exe().unwrap();
    current_dir.pop();
    let current_path = current_dir.to_str().unwrap();
    return current_path.to_owned() + COMMAND_FILE_NAME;
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Ladder",
    about = "A small tool to store useful commands and paste them in to clipboard to relieve your brain.
    Simply just ladder!"
)]
struct Cli {
    #[structopt(short = "a", long = "add", help = "Command to save")]
    pub command_text: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub numeric_representation: i32,
    pub description: String,
    pub value: String,
}
