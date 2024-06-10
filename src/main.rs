use mysql::*;
use mysql::prelude::*;
use std::error::Error;
use std::io;
mod table;

fn main() -> Result<(), Box<dyn Error>> {
    // ask for the DB username
    let mut db_username = String::new();
    println!("DB Username: ");
    io::stdin()
        .read_line(&mut db_username)
        .expect("Failed to read input");

    // ask for the DB password
    let mut db_password = String::new();
    println!("DB Password: ");
    io::stdin()
        .read_line(&mut db_password)
        .expect("Failed to read input");

    // ask for the DB table
    let mut db_table = String::new();
    println!("DB Table: ");
    io::stdin()
        .read_line(&mut db_table)
        .expect("Failed to read input");

    // build the Pool URL using the input values the user has provided
    // https://stackoverflow.com/questions/77823790/how-to-format-string-passed-to-poolnew-of-mysql-crate-in-rust
    let url = format!("mysql://{db_username}:{db_password}@localhost:3306/{db_table}");
    
    // Create a connection pool
    let pool = Pool::new(url.as_str())?;

    // Get a connection from the pool
    let mut conn = pool.get_conn()?;

    // ask what the user wants to do
    let mut action_input = String::new();
    println!("Do you want to:\n1. Add a Task\n2. List existing Tasks ");
    io::stdin()
        .read_line(&mut action_input)
        .expect("Failed to read input");

    // make it a u32 and validate it
    let action_input: i32 = match action_input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Shutting down..."),
    };

    // we only need to create the inputs if the user wants to add a Task
    if action_input == 1 {
        // Define the query to insert a row into the table
        let query = "INSERT INTO tasks (name, description) VALUES (?, ?)";

        // create input to set the task name
        let mut name_input = String::new();
        println!("Task Name: ");
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read input");

        // create input to set the task description
        let mut description_input = String::new();
        println!("Task Description: ");
        io::stdin()
            .read_line(&mut description_input)
            .expect("Failed to read input");

        // Define the values to insert
        let values = vec![name_input, description_input];

        // Execute the query
        conn.exec_drop(query, values)?;

        println!("\nRow inserted successfully!\n");
    }

    // Define the query to select rows from the table
    let query = "SELECT * FROM tasks";

    // Execute the query and fetch the results
    let results: Vec<Row> = conn.query(query)?;

    table::print_table(results);


    Ok(())
}