use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.expect("error: unable read the input");
        let words: Vec<&str> = input.trim().split_whitespace().collect();
        match words[0] {
            "Add" => {
                employees.entry(words[3].to_string()).or_default().push(words[1].to_string());
            },
            "List" => {
                    for name in employees.get(words[1]).unwrap() {
                        println!("{}", name);
                    }
            },
            "All" => {
                for (dept, names) in employees.clone().into_iter() {
                    println!("{}",dept);
                    for name in names {
                        println!("{}", name);
                    }
                    println!("");
                }
            },
            "Quit" => {
                break;
            },
            _ => {
                println!("Invalid command!");
            }
        }
    }
}
