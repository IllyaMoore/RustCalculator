# Rust Calculator Project

This project implements a calculator in Rust that performs arithmetic operations either from user input via the keyboard or from operations read from a file. The user can choose between two input modes: entering operations manually or reading them from the `operations.txt` file.

## Features
- Perform basic arithmetic operations (`+`, `-`, `*`, `/`).
- Input operations manually via the keyboard.
- Read and execute operations from a text file (`operations.txt`).
- Handle invalid inputs and errors gracefully.

## Usage
1. Choose whether to input operations manually or from a file.
2. If choosing manual input, enter operations like `3 + 4` and type `q` to stop.
3. If choosing file input, ensure the file contains operations in the format `num1 operator num2`.
