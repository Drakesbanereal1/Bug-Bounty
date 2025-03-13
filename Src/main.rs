use bug_bounty::{utils, burp_suite, ghidra, klee, z3, full_code}; // Import modules

fn main() {
    println!("Bug Bounty Project Running...");
    utils::log_action("Bug Bounty tool started");

    loop {
        println!("\n----------------------------------");
        println!("Menu options:");
        println!("1) Run Full Code");
        println!("2) Run Burp Suite");
        println!("3) Run KLEE");
        println!("4) Run Z3");
        println!("5) Launch Ghidra");
        println!("6) Exit (Closes Ghidra if running)");

        let choice = utils::prompt_user();

        match choice.as_str() {
            "1" => full_code::run_full_code(),
            "2" => burp_suite::run_burp_suite(),
            "3" => klee::run_klee(),
            "4" => z3::run_z3(),
            "5" => ghidra::run_ghidra(),
            "6" => {
                exit_program();
                break;
            }
            _ => println!("Invalid option. Please enter a number between 1 and 6."),
        }
    }
}

// Function to handle exiting the program
fn exit_program() {
    println!("Closing Ghidra (if running) and exiting...");
    ghidra::close_ghidra();
    println!("Goodbye!");
}
