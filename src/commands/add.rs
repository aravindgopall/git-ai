use crate::commit::commit_changes;
use crate::config::GIT_AI_CONFIG;
use crate::utils::{detect_language, get_combined_ignores, should_ignore_file};
use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub async fn add_files(all: bool, files: Vec<String>) {
    println!("{}", "🔍 Preparing to add files...".cyan());

    let language = detect_language();
    let auto_ignores = get_combined_ignores(&language);
    let is_added;

    if all {
        is_added = add_all_files(&auto_ignores);
    } else if !files.is_empty() {
        is_added = add_specific_files(&files, &auto_ignores);
    } else {
        is_added = interactive_add(&auto_ignores);
    }

    if is_added && GIT_AI_CONFIG.auto_commit == Some(true) {
        commit_changes(false, false, GIT_AI_CONFIG.ai_enabled == Some(true)).await;
        return;
    }
}

// 🔥 Stage all unstaged files
fn add_all_files(auto_ignores: &[String]) -> bool {
    println!("{}", "📝 Staging all unstaged files...".cyan());

    let output = Command::new("git")
        .arg("status")
        .arg("--short")
        .output()
        .expect("Failed to run git status");

    let status_text = String::from_utf8_lossy(&output.stdout);
    let mut added = false;

    let mut unstaged_files = Vec::new();
    let mut ignored_files = Vec::new();

    for line in status_text.lines() {
        let filename = &line[3..];

        if should_ignore_file(filename, auto_ignores) {
            ignored_files.push(filename.to_string());
        } else {
            unstaged_files.push(filename.to_string());
        }
    }

    if unstaged_files.is_empty() {
        println!("{}", "⚠️ Only ignored files found.".yellow());
        println!("Do you still want to stage everything including ignored files? (y/n): ");

        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim().to_lowercase();

        if answer == "y" {
            for file in ignored_files {
                added = true;
                Command::new("git")
                    .arg("add")
                    .arg(&file)
                    .status()
                    .expect("Failed to git add");
                println!("✅ Staged (ignored): {}", file.bright_red());
            }
        } else {
            println!("{}", "🛑 Staging cancelled.".red());
        }
        return added;
    }

    for file in unstaged_files {
        added = true;
        Command::new("git")
            .arg("add")
            .arg(&file)
            .status()
            .expect("Failed to git add");
        println!("✅ Staged: {}", file.bright_green());
    }
    return added;
}

fn add_specific_files(files: &[String], auto_ignores: &[String]) -> bool {
    let mut is_added = false;
    for file in files {
        if should_ignore_file(file, auto_ignores) {
            println!(
                "⚠️ File '{}' matches ignore patterns.",
                file.bright_yellow()
            );
            println!("Do you still want to stage it? (y/n): ");

            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).unwrap();
            let answer = answer.trim().to_lowercase();

            if answer != "y" {
                println!("🛑 Skipped ignored file: {}", file.bright_red());
                continue;
            }
        }
        is_added = true;

        Command::new("git")
            .arg("add")
            .arg(file)
            .status()
            .expect("Failed to git add");

        println!("✅ Staged: {}", file.bright_green());
    }
    return is_added;
}

// 🔥 Interactive add
fn interactive_add(auto_ignores: &[String]) -> bool {
    println!("{}", "📝 Interactive add: choose files to stage".cyan());

    let output = Command::new("git")
        .arg("status")
        .arg("--short")
        .output()
        .expect("Failed to run git status");

    let status_text = String::from_utf8_lossy(&output.stdout);

    let mut unstaged_files = Vec::new();

    for line in status_text.lines() {
        let filename = &line[3..];

        if should_ignore_file(filename, auto_ignores) {
            continue;
        }

        unstaged_files.push(filename.to_string());
    }

    if unstaged_files.is_empty() {
        println!("{}", "✅ No unstaged files found!".green());
        return false;
    }

    println!("\nUnstaged files:");
    for (i, file) in unstaged_files.iter().enumerate() {
        println!("{}. {}", i + 1, file);
    }

    println!("\nPick files to stage space seperated (e.g., 1 2 5 or 'all'):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).unwrap();
    let selection = selection.trim();

    if selection == "all" {
        for file in unstaged_files.iter() {
            Command::new("git")
                .arg("add")
                .arg(file)
                .status()
                .expect("Failed to git add");
            println!("✅ Staged: {}", file.bright_green());
        }
        return true;
    } else {
        let picks: Vec<&str> = selection.split(' ').collect();
        let mut added = false;
        for pick in picks {
            if let Ok(index) = pick.trim().parse::<usize>() {
                if index > 0 && index <= unstaged_files.len() {
                    let file = &unstaged_files[index - 1];
                    Command::new("git")
                        .arg("add")
                        .arg(file)
                        .status()
                        .expect("Failed to git add");
                    println!("✅ Staged: {}", file.bright_green());
                    added = true;
                } else {
                    println!("❌ Invalid selection: {}", pick);
                }
            } else {
                println!("❌ Invalid input: {}", pick);
            }
        }
        return added;
    }
}
