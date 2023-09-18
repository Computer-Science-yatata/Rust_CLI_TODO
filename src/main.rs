#![allow(unused_parens)]
#![allow(while_true)]




use console;
use std;
//use serde::{Serialize}; // Import the Serialize trait
use serde_json;
use serde;
//use serde_derive;

// Define your Value struct with #[derive(Serialize)]
#[derive(serde::ser::Serialize)]
struct Value {
    id: i32,
    name: String,
    description: String,
    date_limit:String,
    done: String,
}


impl Value {
    

    pub fn new () -> Value{


        return Value {
            id: 0,
            name: String::new(),
            description: String::new(),
            date_limit: String::new(),
            done: "Not Done".to_string(),
        };


    }
}





const FILENAME:&str = "todos.txt";




fn add_todos(terminal: &console::Term, todo_list: &mut std::collections::HashMap<i32, Value>) -> bool {




    let mut todo: Value = Value::new();
    let mut input:Result<String, std::io::Error>;

    while true {


        println!("Enter TODO Name: ");

        input = terminal.read_line();

        match input {
            Ok(_) => {
                todo.name = input.unwrap().trim().to_string();
                break;
            }
            Err(err) => eprintln!("Error reading input: {}", err),
        }
    }



    while (true){

        
        println!("Enter TODO Description: ");
        input = terminal.read_line();

        match input {

            Ok(_) => {
                todo.description = input.unwrap().trim().to_string();
                break;
            }
            Err(err) => eprintln!("Error reading input: {}", err),
        }
    }


    while (true){

        
        println!("Enter TODO Date Limit: ");
        input = terminal.read_line();

        match input {

            Ok(_) => {
                todo.date_limit = input.unwrap().trim().to_string();
                break;}
            Err(err) => eprintln!("Error reading input: {}", err),
        }
    }




    todo.id = get_higher_id(&todo_list) + 1;




    match todo_list.insert(todo.id, todo) {
        
        Some(_) => {
            println!("Error. TODO Already exists");
            return false;
        }
        None => {
            println!("Todo added successfully.");
            show_todo(todo_list.get(&(todo_list.len() as i32)).unwrap());
            return true;
        }
    }


}



fn show_todo(todo:&Value) -> (){


    println!("ID: {}, Name: {}, Description: {}, Date Limit: {}, Done: {}", todo.id, todo.name, todo.description, todo.date_limit, todo.done);



}



fn list_todos (todo_hashmap: &std::collections::HashMap<i32, Value>) -> (){




    for value in todo_hashmap{

        show_todo(value.1);
    }



}


fn get_higher_id (todo_hasmap: &std::collections::HashMap<i32, Value>) -> i32{


    let mut id:i32 = 0;

    for (key, value) in todo_hasmap.iter() {
        
        if (id < *key){
            id = *key;
        }
    }

    return id;

}


fn mark_as_done(terminal: &console::Term, todo_hashmap: &mut std::collections::HashMap<i32, Value>) -> (){


    

    while (true){

        match terminal.write_str("Enter TODO ID (0 for Exit): "){
                
            Ok(_) => {} // No error, do nothing
            Err(err) => eprintln!("Error writing line: {}", err),
        }
    
    
        let input: Result<String, std::io::Error> = terminal.read_line();

        

        match input {

            Ok(ref input_ok) => {


                let value: Option<&mut Value>;

                match input_ok.parse::<i32>() {

                    Ok(input_parsed) => {

                        if (input_parsed == 0){
                            break;
                        }

                        value = todo_hashmap.get_mut(&input_parsed);

                        match value {
                
                            Some(value_unwraped) => {
                                value_unwraped.done = "Done".to_string();
                                break;
                            }
                            None => {
                                
                                eprintln!("ID Not Found");
                            }
        
                        }
                    }
                    Err(error_int) => eprintln!("Error parsing to Int32: {}", error_int),
                }            
            }
            Err(ref err) => eprintln!("Error reading input: {}", err),

        }

    }

}



fn remove_todo (terminal: &console::Term, todo_hashmap: &mut std::collections::HashMap<i32, Value>){



    while (true){


        match terminal.write_str("Enter TODO ID (0 for Exit): "){
                
            Ok(_) => {} // No error, do nothing
            Err(err) => eprintln!("Error writing line: {}", err),
        }




        let input: Result<String, std::io::Error> = terminal.read_line();

        

        match input {

            Ok(ref input_ok) => {


            

                match input_ok.parse::<i32>() {

                    Ok(input_parsed) => {

                        if (input_parsed == 0){
                            break;
                        }


                        match todo_hashmap.remove(&input_parsed) {
                
                            Some(value_removed) => {
                                println!("Element with key {} removed.", input_parsed);
                                break;
                            }
                            None => {
                                
                                eprintln!("ID Not Found");
                            }
        
                        }
                    }
                    Err(error_int) => eprintln!("Error parsing to Int32: {}", error_int),
                }            
            }
            Err(ref err) => eprintln!("Error reading input: {}", err),

        }







    }

}


fn load_todos() -> (){

}

fn save_todos(todo_hashmap: &std::collections::HashMap<i32, Value>) -> Result<String, serde_json::Error> {



    let json_str = serde_json::to_string(todo_hashmap)?;


    return Ok(json_str)

}


fn main() {


    let mut todos_hashmap: std::collections::HashMap<i32, Value> = std::collections::HashMap::new();
    let terminal: console::Term = console::Term::stdout();
    terminal.clear_screen().unwrap();


    match terminal.write_line("TODO Terminal CLI Initiated\n######################################\n"){
            
        Ok(_) => {} // No error, do nothing
        Err(err) => eprintln!("Error writing line: {}", err),
    }


    let file:Result<std::fs::File, std::io::Error> = std::fs::File::open(FILENAME);

    match file {
        Ok(_) => {} // No error, do nothing
        Err(err) => eprintln!("Error Opening File: {}", err),
    }


    let mut input: Result<String, std::io::Error>;


    while (true){



        let options = ["Add TODO", "Mark as Done", "Remove TODO", "List TODO", "Exit"];

        for (index, option) in options.iter().enumerate(){

            match terminal.write_line(format!("{} - {}", index, option).as_str()){
                
                Ok(_) => {} // No error, do nothing
                Err(err) => eprintln!("Error writing line: {}", err),
            }
        }



        match terminal.write_str("Enter Option: "){
                
            Ok(_) => {} // No error, do nothing
            Err(err) => eprintln!("Error writing line: {}", err),
        }






        input = terminal.read_line();
    
        match input {

            Ok(_) => {
                
                match input.unwrap().trim(){

                    "0" => {
                        println!("Adding TODO");
                        add_todos(&terminal, &mut todos_hashmap);
                        
                    }
                    "1" => {
                        println!("Marking as Done"); 
                        mark_as_done(&terminal, &mut todos_hashmap);
                    }
                    "2" => {println!("Removing TODO"); 
                        remove_todo(&terminal, &mut todos_hashmap);
                    }
                    "3" => {
                        println!("Listing All TODOs");
                        list_todos(&todos_hashmap);
                    }
                    "4" => {
                        println!("Exiting");
                        break;
                    }
                    _ => {
                        println!("Invalid choice. Please select a valid option.")
                    }

                }
            }
            Err(err) => eprintln!("Error reading input: {}", err),

        }
    }












}

  

