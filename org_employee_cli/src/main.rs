// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use Command::{AddEmployee, Exit, Invalid};

fn main() {
    println!("Welcome to Org Employee CLI. Enter a command.");
    io::stdout().flush().unwrap();

    run(io::stdin().lock(), io::stdout().lock());
}

fn run<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut company = Company::new();

    for line in input.lines() {
        let command = line.expect("Failed to read line");
        let command = command.trim();

        match parse_command(command) {
            AddEmployee { name, department } => {
                company.add(name.clone(), department.clone());
                writeln!(output, "Added {} to {}.", name, department).unwrap();
            }
            Exit => {
                writeln!(output, "Goodbye!").unwrap();
                break;
            }
            Invalid => {
                writeln!(output, "Invalid command {command}.").unwrap();
            }
        }
    }
}

enum Command {
    AddEmployee { name: String, department: String },
    Exit,
    Invalid,
}

fn parse_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts[0].to_lowercase() == "add" && parts[2].to_lowercase() == "to" {
        let name = parts[1].to_string();
        let department = parts[3..].join(" ");
        AddEmployee { name, department }
    } else if command.to_lowercase() == "exit" {
        Exit
    } else {
        Invalid
    }
}

#[derive(Debug)]
struct Company {
    employees: HashMap<String, Vec<String>>,
}
impl Company {
    fn new() -> Self {
        Self {
            employees: HashMap::new(),
        }
    }

    fn add(&mut self, employee: String, department: String) {
        self.employees
            .entry(department.clone())
            .or_default()
            .push(employee.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_integration_add_employee() {
        let input_data = "add Alice to Engineering\nadd Bob to Sales\nexit\n";
        let mut output = Vec::new();

        run(Cursor::new(input_data), &mut output);

        assert_equals(
            output,
            "Added Alice to Engineering.\nAdded Bob to Sales.\nGoodbye!\n",
        );
    }

    fn assert_equals(output: Vec<u8>, expected_output: &str) {
        let actual_output = String::from_utf8(output).expect("Failed to convert output to String");
        assert_eq!(actual_output, expected_output);
    }
}
