use std::fs::File;
use std::io::{self, BufRead};

pub fn read_operations_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let operations: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(operations)
}

pub fn process_operation(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid format. Use 'num1 operator num2'.".to_string());
    }

    let num1: f64 = parts[0].parse().map_err(|_| "Invalid number for the first operand.")?;
    let num2: f64 = parts[2].parse().map_err(|_| "Invalid number for the second operand.")?;

    match parts[1] {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err("Unsupported operation.".to_string()),
    }
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

