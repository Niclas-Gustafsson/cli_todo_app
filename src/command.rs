use std::fs::File;
use std::io::*;
use std::process;

use crate::todo::Todo;
use colored::*;

#[derive(PartialEq, Debug)]
pub enum Command {
    Create,
    Read,
    Update,
    Delete,
    Quit,
}

impl Command {
    pub fn parse_command(input: &str) -> Option<Command> {
        match input {
            "create" => Some(Command::Create),
            "read" => Some(Command::Read),
            "update" => Some(Command::Update),
            "delete" => Some(Command::Delete),
            "quit" => Some(Command::Quit),
            "q" => Some(Command::Quit),
            _ => None,
        }
    }
    pub fn quit_program() {
        // println!("Are you sure you want to quit? y/n");
        process::exit(0);
    }

    pub fn command_loop(file_path: &str) {
        loop {
            let mut user_input = String::new();
            println!(
                "{}",
                "What would you like to do? (create, read, update, delete)".bright_blue()
            );
            stdin()
                .read_line(&mut user_input)
                .expect("failed to read line.");

            println!("You entered: {}", &user_input);

            match Command::parse_command(user_input.trim()) {
                Some(Command::Create) => Todo::create(file_path),
                Some(Command::Read) => Todo::read(),
                Some(Command::Update) => Todo::update(file_path),
                Some(Command::Delete) => Todo::delete(file_path),
                Some(Command::Quit) => Command::quit_program(),
                None => println!(
                    "{}",
                    "Invalid input, try again or type \"quit\" to exit the program.\n\n".red()
                ),
            }
        }
    }
}
