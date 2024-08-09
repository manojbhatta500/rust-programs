use std::io;

fn main() {
    let basic_salary = match get_user_input("Enter your basic salary: ") {
        Some(value) => value,
        None => {
            println!("Invalid input for basic salary.");
            return;
        }
    };

    let allowances = match get_user_input("Enter your allowances: ") {
        Some(value) => value,
        None => {
            println!("Invalid input for allowances.");
            return;
        }
    };

    let total_salary = basic_salary + allowances;

    println!("Your total salary is: ${:.2}", total_salary);
}

fn get_user_input(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
