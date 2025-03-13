mod lib;  // Import lib.rs

fn main() {
    lib::print_welcome_message();
    lib::utils::log_action("Bug Bounty tool started");

    loop {
        println!("----------------------------------");
        println!("Menu options are:");
        println!("1) Full code");
        println!("2) Burp Suite");
        println!("3) KLEE");
        println!("4) Z3");
        println!("5) Ghidra");
        println!("6) Exit");
        println!("----------------------------------");

        let choice = lib::utils::prompt_user();

        match choice.as_str() {
            "1" => println!("Running full code..."),
            "2" => lib::burp_suite::run_burp_suite(),
            "3" => lib::klee::run_klee(),
            "4" => lib::z3::run_z3(),
            "5" => lib::ghidra::run_ghidra(),
            "6" => {
                lib::exit_program();
                break;
            }
            _ => println!("Invalid option. Please enter a number between 1 and 6."),
        }
    }
}
