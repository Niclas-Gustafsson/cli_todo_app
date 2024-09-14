use chrono::{DateTime, Local};
use colored::*;
// use serde_json::Error;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::*;
// use std::io::{BufWriter, Write};
use std::path::Path;

use crate::command::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn create(file_path: &str) {
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

        let mut todos = Todo::read_from_file(file_path).unwrap();
        let mut id = todos.len() as u16;
        println!("Id before comparison: {}", id);

        if id == 0 {
            id = 1;
        } else {
            id += 1;
        }
        let todo = Todo::new(&mut id, &title, &body);

        todos.push(todo);

        Todo::write_to_file(file_path, &todos);
    }

    pub fn read() {
        let file_name = "todos.json";

        let todos = Todo::read_from_file(&file_name).unwrap();

        println!("Listing all todos: ");

        for todo in todos {
            println!("{:?}", todo);
        }
    }

    pub fn update(file_path: &str) {
        // read from file and parse to vec
        let mut todos = Todo::read_from_file(file_path).unwrap();

        let updated_at: DateTime<Local> = Local::now();
        let updated_at = updated_at.format("%Y-%m-%d %H:%M:%S").to_string();
        // read and parse input from user. Should be a number (id)
        println!("parsed todos: {:?}", todos);
        //ask user which todo to modify by id
        let mut id = String::new();

        let mut operation = String::new();
        //let mut body = String::new();

        println!("{}", "Enter the id of todo to update".bright_blue());
        stdin().read_line(&mut id).expect("failed to read line.");
        id = id.trim().to_string();
        let id = id.parse::<u16>().ok();

        let todo_update_index: Option<usize> =
            id.and_then(|id| todos.iter().position(|item| item.id == id));
        //let todo_update_index = todos.iter().position(|&x| x.id == id).unwrap();
        let todo_to_update = &mut todos[todo_update_index.unwrap()];
        println!("Todo to update: {:?}", todo_to_update);

        println!("Todo to update index: {:?}", todo_update_index.unwrap());
        // ask user which field? (title or body) to update. Or have them enter it all in one go?
        println!(
            "{}",
            "Enter the new values in the format: <Title>, <Body> separated by ','".bright_blue()
        );
        stdin()
            .read_line(&mut operation)
            .expect("Failed to read line.");
        let split_ops: Vec<&str> = operation.trim().split(",").collect();

        todo_to_update.title = split_ops[0].to_string();
        todo_to_update.body = split_ops[1].to_string();
        todo_to_update.updated_at = updated_at;

        let todo_clone = todo_to_update.clone();

        let _ = std::mem::replace(&mut todos[todo_update_index.unwrap()], todo_clone);
        // println!("Todos after update: {:?}", updated_todos_vec);
        println!("Todos after update: {:?}", todos);
        Todo::write_to_file(file_path, &todos);
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
