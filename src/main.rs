mod file_operations;
use std::io::{self, Write};

fn main() {
    // console
    println!("Choose input method:");
    println!("1. Enter operations manually");
    println!("2. Read operations from file");

    let mut choice = String::new();
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            loop {
                let mut input = String::new();
                print!("Enter an operation (on alike format: 3 + 4) or 'q' to exit:\nSupported operations: (+ - / *) ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();

                let input = input.trim();
                if input == "q" {
                    break;
                }

                match file_operations::process_operation(input) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        "2" => {
            // file processing 
            let file_path = "src/examples.txt";
            match file_operations::read_operations_from_file(file_path) {
                Ok(operations) => {
                    for operation in operations {
                        match file_operations::process_operation(&operation) {
                            Ok(result) => println!("Result of {} = {}", operation, result),
                            Err(e) => println!("Error processing {} {}", operation, e),
                        }
                    }
                }
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        _ => println!("Invalid choice. Please enter '1' or '2'."),
    }
}
