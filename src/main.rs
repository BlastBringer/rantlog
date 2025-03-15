use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, Write};
use chrono::{Local, Datelike, Timelike};
use colored::*;

// Change this to your preferred directory
const LOG_DIR: &str = "/home/rahul/Documents/rant_logs";  

fn current_date() -> String {
    let now = Local::now();
    format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day()) // YYYY-MM-DD
}

fn timestamp() -> String {
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();
    let ampm = if hour < 12 { "AM" } else { "PM" };
    let hour = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour };

    format!("{:02}:{:02}:{:02} {}", hour, minute, second, ampm)
}

fn log_to_file(rant: &str) -> std::io::Result<String> {
    let date = current_date();
    let timestamp = timestamp();
    
    // Define folder and file path
    let file_path = format!("{}/{}.txt", LOG_DIR, date);

    // Create the folder if it doesn't exist
    create_dir_all(LOG_DIR)?;

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;

    // Write log entry
    let log_entry = format!("[{}] {}\n", timestamp, rant);
    file.write_all(log_entry.as_bytes())?;

    Ok(file_path)
}

fn main() {
    println!("{}", "                 🔥 Welcome to the Rant Logger! 🔥                ".bold().red().on_black());

    loop {
        let time = timestamp();
        println!("{}", format!("Current time: {}", time).yellow().bold());
        println!("{}", "Type your rant. Type 'END' to finish, or 'EXIT' to quit:".blue().bold());

        let mut paragraph = String::new();

        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read line");
            let line = line.trim();

            if line.eq_ignore_ascii_case("END") {
                break;
            } else if line.eq_ignore_ascii_case("EXIT") {
                println!("{}", " Exiting... Your rants are saved! ".green().bold());
                return;
            } else {
                paragraph.push_str(line);
                paragraph.push('\n');
            }
        }

        if !paragraph.trim().is_empty() {
            match log_to_file(&paragraph) {
                Ok(path) => println!("{}", format!("✅ Rant logged successfully at: {}", path).green().bold()),
                Err(e) => eprintln!("{}", format!("❌ Failed to log: {}", e).red().bold()),
            }
        }
    }
}
