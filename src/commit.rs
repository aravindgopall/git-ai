use crate::ai::generate_commit_message;
use crate::ai::suggest_commit_message;
use colored::*;
use std::io::{self, Write};
use std::process::Command; // fallback

pub async fn commit_changes(amend: bool, reword: bool, ai: bool) {
    if amend {
        commit_amend().await;
        return;
    }
    if reword {
        commit_reword();
        return;
    }
    if ai {
        commit_with_ai().await;
        return;
    }

    normal_commit();
}

async fn commit_with_ai() {
    println!("{}", "🤖 Generating commit message with AI...".cyan());

    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("Failed to read staged diff");

    let diff_text = String::from_utf8_lossy(&output.stdout);

    if diff_text.trim().is_empty() {
        println!(
            "{}",
            "⚠️ No staged changes found. Please stage files first!".yellow()
        );
        return;
    }

    match generate_commit_message(&diff_text).await {
        Ok(suggested) => {
            println!(
                "\n✨ AI Suggested Commit Message: {}",
                suggested.bright_magenta()
            );
            println!("Use this message? (y = yes, n = no, q = quit)");

            let mut answer = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut answer).unwrap();
            let answer = answer.trim().to_lowercase();

            match answer.as_str() {
                "y" => {
                    run_git_commit(&suggested);
                }
                "n" => {
                    println!("{}", "📝 Enter your custom commit message:".cyan());
                    let mut custom_message = String::new();
                    io::stdin().read_line(&mut custom_message).unwrap();
                    run_git_commit(custom_message.trim());
                }
                "q" => {
                    println!("{}", "❌ Commit cancelled.".yellow());
                }
                _ => {
                    println!("{}", "❌ Invalid choice. Commit aborted.".red());
                }
            }
        }
        Err(e) => {
            println!("ai commit is not done because of: {}", e);
            normal_commit();
        }
    }
}

fn normal_commit() {
    let suggested = suggest_commit_message();
    println!(
        "\n✨ Suggested Commit Message: {}",
        suggested.bright_magenta()
    );
    println!("Use this message? (y/n/q)");

    let mut answer = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim().to_lowercase();

    match answer.as_str() {
        "y" => {
            run_git_commit(&suggested);
        }
        "n" => {
            println!("{}", "📝 Enter your custom commit message:".cyan());
            let mut custom_message = String::new();
            io::stdin().read_line(&mut custom_message).unwrap();
            run_git_commit(custom_message.trim());
        }
        "q" => {
            println!("{}", "❌ Commit cancelled.".yellow());
        }
        _ => {
            println!("{}", "❌ Invalid choice. Commit aborted.".red());
        }
    }
}

fn run_git_commit(message: &str) {
    let commit_status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to run git commit");

    if commit_status.success() {
        println!("{}", "✅ Commit successful!".green());
    } else {
        println!("{}", "❌ Commit failed!".red());
    }
}

async fn commit_amend() {
    println!("📝 Preparing to amend last commit...");

    let status = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .status()
        .expect("Failed to check staged changes");

    if status.success() {
        // Nothing staged, just reword
        println!("⚠️ No staged changes found.");
        println!("📝 Do you want to reword the last commit message? (y/n): ");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim().to_lowercase();

        if answer == "y" {
            commit_reword();
        } else {
            println!("{}", "❌ Amend cancelled.".yellow());
        }
    } else {
        println!("✨ Staged changes found. Amending into last commit...");

        let suggested = suggest_commit_message();
        println!(
            "\n✨ Suggested Commit Message: {}",
            suggested.bright_magenta()
        );
        println!("Use this message? (y = yes, n = no, q = quit)");

        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim().to_lowercase();

        match answer.as_str() {
            "y" => {
                run_git_commit_amend(&suggested);
            }
            "n" => {
                println!("{}", "📝 Enter your custom amend commit message:".cyan());
                let mut custom_message = String::new();
                std::io::stdin().read_line(&mut custom_message).unwrap();
                run_git_commit_amend(custom_message.trim());
            }
            "q" => {
                println!("{}", "❌ Commit amend cancelled.".yellow());
            }
            _ => {
                println!("{}", "❌ Invalid choice. Aborting.".red());
            }
        }
    }
}

fn commit_reword() {
    println!("📝 Rewording last commit...");

    println!("{}", "📝 Enter the new commit message:".cyan());
    let mut custom_message = String::new();
    std::io::stdin().read_line(&mut custom_message).unwrap();

    let commit_status = Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("-m")
        .arg(custom_message.trim())
        .status()
        .expect("Failed to amend commit message");

    if commit_status.success() {
        println!("{}", "✅ Commit message updated!".green());
    } else {
        println!("{}", "❌ Commit reword failed!".red());
    }
}

fn run_git_commit_amend(message: &str) {
    let commit_status = Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to amend commit");

    if commit_status.success() {
        println!("{}", "✅ Amend successful!".green());
    } else {
        println!("{}", "❌ Amend failed!".red());
    }
}
