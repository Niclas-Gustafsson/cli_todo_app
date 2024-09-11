use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local};
use colored::*;
// use serde_json::Error;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::*;
// use std::io::{BufWriter, Write};
use std::path::Path;
use std::process;

use crate::command::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub body: String,
    pub updated_at: String,
    pub created_at: String,
}

impl Todo {
    fn new(title: &String, body: &String) -> Self {
        let created_at: DateTime<Local> = Local::now();
        let updated_at: DateTime<Local> = Local::now();

        let created_at = created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let updated_at = updated_at.format("%Y-%m-%d %H:%M:%S").to_string();

        Self {
            title: title.to_string(),
            body: body.to_string(),
            updated_at,
            created_at,
        }
    }

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
        stdin().read_line(&mut body).expect("Unable to read line.");
        //trim body
        body = body.trim().to_string();

        if body == "quit" {
            Command::quit_program();
        }

        //create new Todo.
        let todo = Todo::new(&title, &body);

        let file_name = "todos.json";
        let mut todos = Todo::read_from_file(&file_name).unwrap();

        todos.push(todo);

        Todo::write_to_file(&file_name, &todos);
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

    //Helper funcs
    fn write_to_file(file_path: &str, todos: &Vec<Todo>) {
        let json = serde_json::to_string_pretty(todos).expect("Unable to serialize to JSON");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Unable to open file");

        file.write_all(json.as_bytes())
            .expect("Unable to write file");
    }

    fn read_from_file(file_path: &str) -> Result<Vec<Todo>> {
        if !Path::new(file_path).exists() {
            return Ok(vec![]);
        }

        let mut file = File::open(file_path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        if contents.is_empty() {
            Ok(vec![])
        } else {
            match serde_json::from_str::<Vec<Todo>>(&contents) {
                Ok(todos) => Ok(todos),
                Err(_) => {
                    println!("Failed to parse JSON");
                    Ok(vec![])
                }
            }
        }
    }
}
