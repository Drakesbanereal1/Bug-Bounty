pub mod utils;
pub mod burp_suite;
pub mod ghidra;
pub mod klee;
pub mod z3;
pub mod full_code;

 // ✅ Ensure this matches the actual `z3.rs` file name

 mod lib;  // Import the shared library module

 fn main() {
     println!("Bug Bounty Project Running...");
     lib::utils::log_action("Bug Bounty tool started");
 }
 
pub fn exit_program() {
    println!("Closing Ghidra (if running) and exiting...");
    ghidra::close_ghidra();
    println!("Goodbye!");
}
