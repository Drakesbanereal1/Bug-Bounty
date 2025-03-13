// Set the correct path to ghidraRun.bat make sure that you follow the path correctly
use std::process::{Command, Stdio};

pub fn run_ghidra() {
    let ghidra_path = "C:\\Users\\steve\\OneDrive\\Desktop\\CIS class\\CISS-301\\Bug Bounty\\Bug-Bounty\\Ghidra\\ghidra_11.3.1_PUBLIC_20250219\\ghidra_11.3.1_PUBLIC\\ghidraRun.bat";

    println!("Launching Ghidra...");

    match Command::new(ghidra_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(_) => println!("Ghidra launched successfully!"),
        Err(e) => println!("Failed to launch Ghidra: {}", e),
    }
}

pub fn close_ghidra() {
    println!("Closing Ghidra...");

    match Command::new("taskkill")
        .args(&["/IM", "java.exe", "/F"])
        .output()
    {
        Ok(_) => println!("Ghidra (Java process) closed successfully."),
        Err(e) => println!("Failed to close Ghidra: {}", e),
    }
}
