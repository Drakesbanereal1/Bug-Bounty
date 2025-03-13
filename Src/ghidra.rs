// Set the correct path to ghidraRun.bat make sure that you follow the path correctly
use std::process::{Command, Child};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GHIDRA_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
}

pub fn run_ghidra() {
    println!("Launching Ghidra...");

    let ghidra_path = r"C:\Users\steve\OneDrive\Desktop\CIS class\CISS-301\Bug Bounty\Bug-Bounty\Ghidra\ghidra_11.3.1_PUBLIC_20250219\ghidra_11.3.1_PUBLIC\ghidraRun.bat";

    let process = match Command::new(ghidra_path)
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
    println!("Attempting to close Ghidra...");

    // Kill the batch file process first
    let mut ghidra_proc = GHIDRA_PROCESS.lock().unwrap();
    if let Some(ref mut process) = *ghidra_proc {
        match process.kill() {
            Ok(_) => println!("Ghidra process (bat file) closed successfully."),
            Err(_) => println!("Could not terminate ghidraRun.bat, trying to kill the main Ghidra process..."),
        }
    }

    // Kill the actual Ghidra process (Java or other detected process name)
    let process_names = ["java.exe", "ghidra.exe"]; // Try different process names
    for process_name in process_names.iter() {
        let result = Command::new("taskkill")
            .args(["/F", "/IM", process_name])
            .output();

        match result {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("SUCCESS") {
                    println!("Successfully terminated process: {}", process_name);
                    break; // Exit loop if we successfully kill a process
                }
            }
            Err(_) => println!("Failed to close process: {}", process_name),
        }
    }
}
