// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Cannot read command");
        let command = command.trim();

        if command.to_lowercase() == "exit" {
            break;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts[0].to_lowercase() == "add" && parts[2].to_lowercase() == "to" {
            let employee = parts[1].to_string();
            let department = parts[3..].join(" ");

            company.entry(department.clone()).or_default().push(employee.clone());
            println!("Added {employee} to {department}");
            println!("{:?}", company);
        } else {
            println!("Invalid command {}", parts.join(" "));
            continue;
        }
    }
    println!("Goodbye!");
}
