use colored::*;
use std::io::{self, Write};

pub fn start_interactive_review(diff: String) {
    for (i, line) in diff.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        println!("\n[{}] {}", i + 1, line.bright_yellow());
        print!("Keep this change? (y/n): ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().to_lowercase() == "y" {
            println!("{}", "[Kept]".green());
            println!("{}", line.bright_white());
        } else {
            println!("{}", "[Ignored]".red());
        }
    }
}
