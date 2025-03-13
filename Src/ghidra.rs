use std::process::Command;

pub fn run_ghidra() {
    println!("Launching Ghidra...");

    // Replace with your actual Ghidra installation path
    let ghidra_path = "C:\\Path\\To\\Ghidra\\ghidraRun.bat";

    // Attempt to run Ghidra
    match Command::new(ghidra_path).spawn() {
        Ok(_) => println!("Ghidra launched successfully!"),
        Err(e) => println!("Failed to start Ghidra: {}", e),
    }
}
