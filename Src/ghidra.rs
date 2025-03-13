use std::process::{Command, Child};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GHIDRA_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
}

pub fn run_ghidra() {
    println!("Launching Ghidra...");

    let ghidra_path = r"C:\Users\steve\OneDrive\Desktop\CIS class\CISS-301\Bug Bounty\Bug-Bounty\Ghidra\ghidra_11.3.1_PUBLIC_20250219\ghidra_11.3.1_PUBLIC\ghidraRun.bat";

    let process = match Command::new("cmd")
        .args(["/C", "start", "", ghidra_path])
        .spawn()
    {
        Ok(p) => {
            println!("Ghidra launched successfully!");
            let mut ghidra_proc = GHIDRA_PROCESS.lock().unwrap();
            *ghidra_proc = Some(p);
        }
        Err(e) => {
            println!("Failed to start Ghidra: {}", e);
        }
    };
}

pub fn close_ghidra() {
    let mut ghidra_proc = GHIDRA_PROCESS.lock().unwrap();
    if let Some(ref mut process) = *ghidra_proc {
        match process.kill() {
            Ok(_) => println!("Ghidra closed successfully."),
            Err(e) => println!("Failed to close Ghidra: {}", e),
        }
    } else {
        println!("No active Ghidra process found.");
    }
}
