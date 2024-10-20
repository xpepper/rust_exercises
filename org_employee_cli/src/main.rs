// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;
use std::io::Write;
use Command::{AddEmployee, Exit, Invalid, ListAll, ListAllInDepartment};

fn main() {
    let mut company = Company::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Cannot read command");
        let command = input_line.trim();

        match parse_command(command) {
            AddEmployee { name, department } => {
                company.add(name.clone(), department.clone());
                println!("Added {name} to {department}");
            }
            ListAllInDepartment { department } => {
                let employees = company.all_by(department.clone());
                println!("{:?}", employees);
            }
            ListAll => {
                let employees = company.all();
                println!("{:?}", employees);
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
    ListAllInDepartment { department: String },
    ListAll,
    Exit,
    Invalid,
}

fn parse_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if is_add_employee(&parts) {
        let name = parts[1].to_string();
        let department = parts[3..].join(" ");
        AddEmployee { name, department }
    } else if is_list_in_department(&parts) {
        let department = parts[2..].join(" ");
        ListAllInDepartment { department }
    } else if is_list_all(&parts) {
        ListAll
    } else if command.to_lowercase() == "exit" {
        Exit
    } else {
        Invalid
    }
}

fn is_add_employee(parts: &[&str]) -> bool {
    parts[0].to_lowercase() == "add" && parts[2].to_lowercase() == "to"
}

fn is_list_in_department(parts: &[&str]) -> bool {
    parts[0].to_lowercase() == "list" && parts[1].to_lowercase() == "department"
}

fn is_list_all(parts: &[&str]) -> bool {
    parts[0].to_lowercase() == "list" && parts[1].to_lowercase() == "all"
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

    pub(crate) fn all_by(&self, department: String) -> Vec<String> {
        self.employees
            .get(&department)
            .cloned()
            .unwrap_or_else(Vec::new)
    }

    pub(crate) fn all(&self) -> Vec<String> {
        self.employees
            .keys()
            .flat_map(|department| self.all_by(department.clone()))
            .collect()
    }
}
