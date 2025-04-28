use crate::ai::generate_commit_message;
use crate::ai::suggest_commit_message;
use crate::config::GIT_AI_CONFIG;
use crate::push::push_changes;
use crate::utils::has_staged_changes;

use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub async fn commit_changes(amend: bool, reword: bool, ai: bool) {
    let mut is_committed = false;
    if !has_staged_changes() {
        println!(
            "{}",
            "⚠️ No staged changes found. Please stage files first!".yellow()
        );
    } else if amend {
        is_committed = commit_amend().await;
    } else if reword {
        is_committed = commit_reword();
    } else if ai {
        is_committed = commit_with_ai().await;
    } else {
        is_committed = normal_commit();
    }

    if is_committed {
        if GIT_AI_CONFIG.auto_push == Some(true) {
            push_changes();
            return;
        }
        println!("successfully commit, do you want to push also (y/n)");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().to_lowercase() == "y" {
            push_changes();
        }
    }
}

async fn commit_with_ai() -> bool {
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
        return false;
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
                "y" => run_git_commit(&suggested),
                "n" => {
                    println!("{}", "📝 Enter your custom commit message:".cyan());
                    let mut custom_message = String::new();
                    io::stdin().read_line(&mut custom_message).unwrap();
                    run_git_commit(custom_message.trim())
                }
                "q" => {
                    println!("{}", "❌ Commit cancelled.".yellow());
                    return false;
                }
                _ => {
                    println!("{}", "❌ Invalid choice. Commit aborted.".red());
                    return false;
                }
            }
        }
        Err(e) => {
            println!("ai commit is not done because of: {}", e);
            normal_commit()
        }
    }
}

fn normal_commit() -> bool {
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
        "y" => run_git_commit(&suggested),
        "n" => {
            println!("{}", "📝 Enter your custom commit message:".cyan());
            let mut custom_message = String::new();
            io::stdin().read_line(&mut custom_message).unwrap();
            run_git_commit(custom_message.trim())
        }
        "q" => {
            println!("{}", "❌ Commit cancelled.".yellow());
            return false;
        }
        _ => {
            println!("{}", "❌ Invalid choice. Commit aborted.".red());
            return false;
        }
    }
}

fn run_git_commit(message: &str) -> bool {
    let commit_status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to run git commit");

    if commit_status.success() {
        println!("{}", "✅ Commit successful!".green());
        return true;
    } else {
        println!("{}", "❌ Commit failed!".red());
        return false;
    }
}

async fn commit_amend() -> bool {
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
            commit_reword()
        } else {
            println!("{}", "❌ Amend cancelled.".yellow());
            return false;
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
                return true;
            }
            "n" => {
                println!("{}", "📝 Enter your custom amend commit message:".cyan());
                let mut custom_message = String::new();
                std::io::stdin().read_line(&mut custom_message).unwrap();
                run_git_commit_amend(custom_message.trim())
            }
            "q" => {
                println!("{}", "❌ Commit amend cancelled.".yellow());
                return false;
            }
            _ => {
                println!("{}", "❌ Invalid choice. Aborting.".red());
                return false;
            }
        }
    }
}

fn commit_reword() -> bool {
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
        return true;
    } else {
        println!("{}", "❌ Commit reword failed!".red());
        return false;
    }
}

fn run_git_commit_amend(message: &str) -> bool {
    let commit_status = Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to amend commit");

    if commit_status.success() {
        println!("{}", "✅ Amend successful!".green());
        return true;
    } else {
        println!("{}", "❌ Amend failed!".red());
        return false;
    }
}
