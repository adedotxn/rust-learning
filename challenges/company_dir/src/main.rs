use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!(">>");

        // make sure anything printed to the screen is flushed immediately and unwrap so that the program panics if there is an error
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // read user ipnut and store in string variable
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "quit" {
            break;
        }

        handle_input(input, &mut company);
    }
}

fn handle_input(input: &str, company: &mut HashMap<String, Vec<String>>) {
    if input.starts_with("Add ") {
        // split the input into substrings at every whitespace occurence and store in a vector
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() == 4 && parts[2] == "to" {
            let employee = parts[1].to_string();
            let department = parts[3].to_string();

            add_employee(company, &employee, &department);
        } else {
            println!("Invalid add command. Use format: Add [name] to [department]");
        }
    } else if input == "List all" {
        list_all(company)
    } else if input.starts_with("List ") {
        // creates a string slice from the 5th index to the end so everything after "List "
        let department = &input[5..].trim();
        list_department(company, department);
    } else {
        println!("Unknown command. Available commands:");
        println!("  Add [name] to [department]");
        println!("  List [department]");
        println!("  List all");
        println!("  quit");
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, employee: &str, department: &str) {
    // insert the department and employee into the company hashmap, create new vector if department does not exist
    company
        .entry(department.to_string())
        .or_insert(vec![])
        .push(employee.to_string());

    println!("Added {} to {}", employee, department);
}

fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
    // patterm matching
    if let Some(employees) = company.get(department) {
        let mut sorted_employees = employees.clone();

        // had to clone because sort requires a mutable ref but "employees" variable itself is an immutable reference 'cause it was obtained from the hashmap using .get()
        sorted_employees.sort();

        println!("Employees in {} department:", department);
        for employee in sorted_employees {
            println!("{}", employee);
        }
    } else {
        println!("No employees found in  {}", department);
    }
}

fn list_all(company: &HashMap<String, Vec<String>>) {
    let mut all_departments: Vec<&String> = company.keys().collect();
    all_departments.sort();

    for department in all_departments {
        list_department(company, department);
    }
}
