// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use Command::{AddEmployee, Exit, Invalid, ListAll, ListAllInDepartment};

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
            ListAllInDepartment { department } => {
                let employees = company.all_in_department(&department);
                writeln!(output, "{:?}", employees).unwrap();
            }
            ListAll => {
                let employees = company.all();
                writeln!(output, "{:?}", employees).unwrap();
            }
            Invalid => {
                writeln!(output, "Invalid command {command}.").unwrap();
            }
        }
    }
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

    pub(crate) fn all_in_department(&self, department: &String) -> Vec<String> {
        let mut employees = self.all_by(department);
        employees.sort();
        employees
    }

    pub(crate) fn all(&self) -> Vec<String> {
        let mut employees: Vec<String> = self
            .employees
            .keys()
            .flat_map(|department| self.all_by(department))
            .collect();

        employees.sort();
        employees
    }

    fn all_by(&self, department: &String) -> Vec<String> {
        self.employees
            .get(department)
            .cloned()
            .unwrap_or_else(Vec::new)
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
