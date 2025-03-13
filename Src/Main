mod burp_suite;
mod klee;
mod z3;
mod full_code;

use std::io;

fn main() {
    loop {
        println!("----------------------------------");
        println!("Hello, this code is for educational purposes only!");
        println!("Menu options are:");
        println!("1) Full code");
        println!("2) Burp Suite");
        println!("3) KLEE");
        println!("4) Z3");
        println!("5) Exit");
        println!("----------------------------------");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => full_code::run_full_code(),
            "2" => burp_suite::run_burp_suite(),
            "3" => klee::run_klee(),
            "4" => z3::run_z3(),
            "5" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please enter a number between 1 and 5."),
        }
    }
}
