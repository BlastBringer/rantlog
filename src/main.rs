use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, Write};
use chrono::{Local, Datelike, Timelike};
use colored::*;
use crossterm::terminal::{self, Clear, ClearType};


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
    let log_entry = format!("[{}]\n{}\n\n", timestamp, rant);
    file.write_all(log_entry.as_bytes())?;

    Ok(file_path)
}

fn print_welcome_message() {
    let width = terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let message = "🔥 Welcome to the Rant Logger! 🔥".bold().red().on_black();
    let padding = (width.saturating_sub(32)) / 2; // Adjust based on message length

    println!("{}", " ".repeat(width).on_black()); // Top black bar
    println!("{}{}", " ".repeat(padding), message);
    println!("{}", " ".repeat(width).on_black()); // Bottom black bar
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing screen
    io::stdout().flush().unwrap();
}

fn main() {
    clear_screen();
    print_welcome_message();

    loop {
        let time = timestamp();
        println!("\n{}", format!("⏳ Current time: {}", time).yellow().bold());
        println!("{}", "📝 Type your rant. Type 'END' to finish, or 'EXIT' to quit:".blue().bold());

        let mut paragraph = String::new();

        loop {
            print!(">> "); // Input indicator
            io::stdout().flush().unwrap(); // Ensure ">>" prints before input
            
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read line");
            let line = line.trim();

            if line.eq_ignore_ascii_case("END") {
                break;
            } else if line.eq_ignore_ascii_case("EXIT") {
                println!("\n{}", "👋 Exiting... Your rants are safely saved! 🎤🔥".green().bold());
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
