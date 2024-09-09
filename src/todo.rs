use std::process;
use std::io::*;
use colored::*;
use chrono::{DateTime, Local};


#[derive(PartialEq, Debug)]
pub enum Command {
    Create,
    Read,
    Update,
    Delete,
    Quit
}
#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub body: String,
    pub updated_at: DateTime<Local>,
    pub created_at: DateTime<Local>
}

impl Todo {
 pub fn create() {
    println!("Create method called");
    let mut title = String::new();
    let mut body = String::new();
    println!("Enter a title");
    stdin().read_line(&mut title).expect("failed to read line.");
    title = title.trim().to_string();


    if title == "quit" {
        Command::quit_program();
    }

    stdout().flush().unwrap(); 

    println!("What do you need to do?");
    stdin().read_line(&mut body).expect("failed to read line.");
    //trim body
    body = body.trim().to_string();

    if body == "quit" {
        Command::quit_program();
    }

    println!("You entered title: {}, and body: {}", title, body);

 }
 pub fn read() {
    println!("Read method called")
 }
 pub fn update() {
    println!("Update method called")
 }
 pub fn delete() {
    println!("Delete method called")
 }
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
            _ => None
        }
    }
    pub fn quit_program() {
        // println!("Are you sure you want to quit? y/n");
        process::exit(0);
    }

    pub fn command_loop() {
        loop {

            let mut user_input = String::new();
            println!("What would you like to do? (create, read, update, delete)");
            stdin().read_line(&mut user_input).expect("failed to read line.");
        
        
            println!("You entered: {}", &user_input);
        
        
            match Command::parse_command(&user_input.trim()) {
                Some(Command::Create) => Todo::create(),
                Some(Command::Read) => Todo::read(),
                Some(Command::Update) => Todo::update(),
                Some(Command::Delete) => Todo::delete(),
                Some(Command::Quit) => {Command::quit_program()},
                None => println!("{}", "Invalid input, try again or type \"quit\" to exit the program.".red()),
            }
        }
    }
}