use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn log_action(action: &str) {
    let timestamp = Local::now();
    let log_entry = format!("[{}] {}\n", timestamp.format("%Y-%m-%d %H:%M:%S"), action);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("bug_bounty.log")
        .expect("Unable to open log file");

    file.write_all(log_entry.as_bytes()).expect("Failed to write to log");
    println!("{}", log_entry.trim());
}
