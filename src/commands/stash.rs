use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub fn stash_handler(command: &str) {
    match command {
        "save" => save_stash(),
        "list" => list_stashes(),
        "pop" => pop_stash(),
        "drop" => drop_stash(),
        _ => {
            println!("{}", "❌ Invalid stash command.".red());
        }
    }
}

// Save a new stash
fn save_stash() {
    println!("{}", "📝 Enter a name for this stash:".cyan());
    print!("> ");
    io::stdout().flush().unwrap();

    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    if message.is_empty() {
        println!("{}", "❌ Stash message cannot be empty.".red());
        return;
    }

    Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to save stash");

    println!(
        "✅ Stashed changes with message: '{}'",
        message.bright_green()
    );
}

// List all stashes
fn list_stashes() {
    println!("{}", "📋 Listing current stashes...".cyan());

    let output = Command::new("git")
        .arg("stash")
        .arg("list")
        .output()
        .expect("Failed to list stashes");

    let list = String::from_utf8_lossy(&output.stdout);

    if list.trim().is_empty() {
        println!("✅ No stashes found.");
    } else {
        for (index, line) in list.lines().enumerate() {
            println!("{}: {}", index, line);
        }
    }
}

// Pop a specific stash
fn pop_stash() {
    list_stashes();

    println!(
        "{}",
        "\n🛠️ Pick a stash number to pop (or type 'cancel')".cyan()
    );
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "cancel" {
        println!("{}", "❌ Pop cancelled.".yellow());
        return;
    }

    if let Ok(index) = input.parse::<usize>() {
        let stash_ref = format!("stash@{{{}}}", index);

        Command::new("git")
            .arg("stash")
            .arg("pop")
            .arg(&stash_ref)
            .status()
            .expect("Failed to pop stash");

        println!("✅ Popped stash {}", stash_ref.bright_green());
    } else {
        println!("{}", "❌ Invalid input. Pop aborted.".red());
    }
}

// Drop (delete) a specific stash
fn drop_stash() {
    list_stashes();

    println!(
        "{}",
        "\n🛠️ Pick a stash number to drop (or type 'cancel')".cyan()
    );
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "cancel" {
        println!("{}", "❌ Drop cancelled.".yellow());
        return;
    }

    if let Ok(index) = input.parse::<usize>() {
        let stash_ref = format!("stash@{{{}}}", index);

        Command::new("git")
            .arg("stash")
            .arg("drop")
            .arg(&stash_ref)
            .status()
            .expect("Failed to drop stash");

        println!("✅ Dropped stash {}", stash_ref.bright_green());
    } else {
        println!("{}", "❌ Invalid input. Drop aborted.".red());
    }
}
