use std::io::*;

use cli_todo::todo::*;

fn main() {
/*     let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y-%m-%d %H:%M:%S");
    println!("{}", custom_format); */

    //Look for storage file, if not there create new storage file
        // write information about the file, if it's empty (has no todos)
    Command::command_loop();
    //If storage file exists, print contents to the terminal and start command prompt/ listen for prompt
  
  /*   println!("Enter what you want to do by naming<operation>, <title>, <content> all seperated by comma. E.g: create, my first todo, create my first todo.");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("failed to read line.");
    println!("You entered: {}", &user_input);

    parse_string(&mut user_input); */
    
}


fn parse_string(input: &mut String) {
    let input_to_str: Vec<&str> = input.as_str().trim().split(",").collect();

    println!("{:?}", input_to_str);
}

