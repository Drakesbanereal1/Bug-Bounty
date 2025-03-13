pub mod utils;  // Import the utilities module

pub mod burp_suite;
pub mod ghidra;
pub mod klee;
pub mod z3;

pub fn print_welcome_message() {
    println!("Welcome to the Bug Bounty Rust Tool!");
}

pub fn exit_program() {
    println!("Closing Ghidra (if running) and exiting...");
    ghidra::close_ghidra();
    println!("Goodbye!");
}
