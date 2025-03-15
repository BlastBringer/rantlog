RantLog

🔥 A Simple Terminal-Based Rant Logger

RantLog is a simple CLI tool written in Rust that allows you to jot down your thoughts and save them as logs. Each rant is timestamped and stored in a folder, with daily logs saved under a separate file.

📥 Installation

Clone the repository

git clone https://github.com/BlastBringer/rantlog.git
cd rantlog

Build the project

cargo build --release

Move the binary to a system path

sudo mv target/release/rantlog /usr/local/bin/rantlog

Make sure it's executable

chmod +x /usr/local/bin/rantlog

🚀 Usage

Run the rantlog command in your terminal:

rantlog

Then, start typing your thoughts.

Type END to finish and save the log.

Type EXIT to quit without saving.

📂 Where Are Logs Saved?

Logs are stored in:

~/Documents/rant_logs/YYYY-MM-DD.txt

For example, if today is March 15, 2025, your log will be saved as:

~/Documents/rant_logs/2025-03-15.txt

If you rant again on the same day, the content is appended to the same file.

🛠 Dependencies

Rust

Chrono (for timestamps)

Colored (for terminal styling)

🛠 Future Improvements

📂 Option to change save directory dynamically.

📝 Search and read past rants.

🌐 Web-based UI for logs.

🏆 Contributing

Feel free to submit issues or pull requests!

