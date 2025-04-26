use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub fn push_changes() {
    println!("🚀 Preparing to push changes...");

    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("Failed to get git remote URL");

    let remote_url = String::from_utf8_lossy(&output.stdout);

    if remote_url.starts_with("git@") {
        println!("🔒 Detected SSH-based remote.");

        println!("🛡️ Do you want to add your SSH key to avoid password prompts? (y/n): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().to_lowercase() == "y" {
            println!("🔑 Starting ssh-agent and adding your key...");

            Command::new("ssh-agent")
                .arg("-s")
                .status()
                .expect("Failed to start ssh-agent");

            Command::new("ssh-add")
                .status()
                .expect("Failed to add SSH key");

            println!("✅ SSH key added. Proceeding to push...");
        } else {
            println!("⚡ Skipping SSH setup. Proceeding to push...");
        }
    } else {
        println!("🌐 HTTPS remote detected. No SSH needed.");
    }

    // Finally push
    println!("🚀 Running git push...");
    let push_status = Command::new("git")
        .arg("push")
        .status()
        .expect("Failed to push");

    if push_status.success() {
        println!("{}", "✅ Pushed successfully!".green());
    } else {
        println!("{}", "❌ Push failed.".red());
    }
}
