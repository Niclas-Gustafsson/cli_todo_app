use cli_todo::command::Command;
use colored::Colorize;
use std::env;
use std::fs::OpenOptions;
use std::path::Path;

// use cli_todo::todo::*;

fn main() -> std::io::Result<()> {
    //Look for storage file, if not there create new storage file
    // let mut file = File::open("todos.json")?;
    let file_name: &str = "todos.json";
    if Path::new(file_name).exists() {
        let current_path = env::current_dir()?;
        println!(
            "{}, from path: {:?}",
            "Storage file loaded".bright_green(),
            current_path
        );

        // file = File::open(file_name)?;
    } else {
        // File doesn't exist, create it
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)?;
        println!("{}", "Created storage file for you".bright_green());
    }

    // write information about the file, if it's empty (has no todos)

    //If storage file exists, print contents to the terminal and start command prompt/ listen for prompt
    Command::command_loop(&file_name);

    Ok(())
}
