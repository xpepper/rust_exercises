// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;
use std::io::Write;
use Command::{AddEmployee, Exit, Invalid};

fn main() {
    let mut company = Company::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Cannot read command");
        let command = command.trim();

        match parse_command(command) {
            AddEmployee { name, department } => {
                company.add(name.clone(), department.clone());
                println!("{:?}", company);
            }
            Exit => break,
            Invalid => {
                println!("Invalid command {}", command);
                continue;
            }
        }
    }
    println!("Goodbye!");
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
        println!("Added {employee} to {department}");
    }
}
