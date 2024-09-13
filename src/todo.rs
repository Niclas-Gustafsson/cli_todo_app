use chrono::{DateTime, Local};
use colored::*;
// use serde_json::Error;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::*;
// use std::io::{BufWriter, Write};
use std::path::Path;

use crate::command::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u16,
    pub title: String,
    pub body: String,
    pub updated_at: String,
    pub created_at: String,
}

impl Todo {
    fn new(id: &u16, title: &String, body: &String) -> Self {
        let created_at: DateTime<Local> = Local::now();
        let updated_at: DateTime<Local> = Local::now();

        let created_at = created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let updated_at = updated_at.format("%Y-%m-%d %H:%M:%S").to_string();

        Self {
            id: *id,
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

        let file_name = "todos.json";
        let mut todos = Todo::read_from_file(&file_name).unwrap();
        let mut id = todos.len() as u16;

        if id == 0 {
            id = 1;
        }
        let todo = Todo::new(&mut id, &title, &body);

        todos.push(todo);

        Todo::write_to_file(&file_name, &todos);
    }

    pub fn read() {
        let file_name = "todos.json";

        let todos = Todo::read_from_file(&file_name).unwrap();

        println!("Listing all todos: ");

        for todo in todos {
            println!("{:?}", todo);
        }
    }

    pub fn update(file_name: &str) {
        // read from file and parse to vec
        let todos = Todo::read_from_file(file_name).unwrap();
        // read and parse input from user. Should be a number (id)
        println!("parsed todos: {:?}", todos);
        //ask user which todo to modify by id
        let mut id = String::new();

        //let mut title = String::new();
        //let mut body = String::new();

        println!("{}", "Enter the id of todo to update".bright_blue());
        stdin().read_line(&mut id).expect("failed to read line.");
        id = id.trim().to_string();
        let id = id.parse::<u16>().ok();

        let found_todo: Option<&Todo> = id.and_then(|id| todos.iter().find(|&item| item.id == id));
        println!("Todo to update: {:?}", found_todo);
        // ask user which field? (title or body) to update. Or have them enter it all in one go?
        // let them enter the updated value
        // update todo and vec, write to file.
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
