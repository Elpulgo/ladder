extern crate clipboard_ext;

use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut numeric_input: i32;

    let mut commands = Vec::new();
    for i in 1..9 {
        commands.push(Command {
            numeric_representation: i,
            description: format!("Description {}", i * 99),
            value: format!("sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-devD {}", i * 1000)
        });
    }

    for command in commands.iter() {
        println!(
            "{}. {} # \t {}",
            command.numeric_representation, command.description, command.value
        );
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
            // println!("{:?}", command);
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(command.value.to_string()).unwrap();
            break;
        } else {
            println!("No ladder found for {}", numeric_input);
            continue;
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub numeric_representation: i32,
    pub description: String,
    pub value: String,
}
