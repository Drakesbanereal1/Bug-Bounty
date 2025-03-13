use std::io::{self, Write};

pub fn log_action(action: &str) {
    println!("[LOG]: {}", action);
}

pub fn prompt_user() -> String {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap(); // Flush output to ensure prompt appears

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    input.trim().to_string()
}
