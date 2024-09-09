use std::process;
use std::io::*;
use colored::*;
use chrono::{DateTime, Local};

use crate::command::Command;



#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub body: String,
    pub updated_at: DateTime<Local>,
    pub created_at: DateTime<Local>
}

impl Todo {
 pub fn create() {
    // println!("Create method called");
    let mut title = String::new();
    let mut body = String::new();
    println!("{}", "Enter a title".bright_blue());
    stdin().read_line(&mut title).expect("failed to read line.");
    title = title.trim().to_string();


    if title == "quit" {
        Command::quit_program();
    }

    stdout().flush().unwrap(); 

    println!("{}", "What do you need to do?".bright_blue());
    stdin().read_line(&mut body).expect("failed to read line.");
    //trim body
    body = body.trim().to_string();

    if body == "quit" {
        Command::quit_program();
    }

    println!("You entered title: {}, and body: {}", title, body);

    //create new Todo.

    //Write to json file (parse, add, write)

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
