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
        println!("6) Exit");mod lib;  // Import the shared library module

        // Bring in functions from modules
        use lib::utils;
        use lib::ghidra;
        use lib::burp_suite;
        use lib::z3;
        use lib::klee;
        use lib::full_code;
        
        fn main() {
            println!("Bug Bounty Project Running...");
            utils::log_action("Bug Bounty tool started");
        
            loop {
                println!("\nSelect an option:");
                println!("1) Run Full Code");
                println!("2) Run Burp Suite");
                println!("3) Run KLEE");
                println!("4) Run Z3");
                println!("5) Launch Ghidra");
                println!("6) Exit (Closes Ghidra if running)");
        
                let mut choice = String::new();
                std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        
                match choice.trim() {
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
        
        pub fn exit_program() {
            println!("Closing Ghidra (if running) and exiting...");
            ghidra::close_ghidra();
            println!("Goodbye!");
        }
        
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
