use colored::*;
use std::io::{self, Write};
use std::process::Command;

pub fn smart_pull() {
    println!("{}", "🔍 Checking Git pull situation...".cyan());

    let mut stashed = false;

    if !is_clean_working_tree() {
        println!("{}", "⚠️ Uncommitted changes detected!".yellow());
        println!("🛡️ Auto-stashing changes before pull...");
        stash_changes();
        stashed = true;
    }

    if is_branch_behind() {
        println!("{}", "📥 Your branch is behind the remote.".yellow());
        show_incoming_commits();
        pull_strategy_decision();
    } else {
        println!("{}", "✅ Branch already up to date.".green());
    }

    if stashed {
        println!("{}", "🛡️ Restoring stashed changes after pull...");
        pop_stash();
    }

    if is_merge_conflict() {
        show_conflict_summary();
    } else {
        println!(
            "{}",
            "✅ Pull and unstash successful. Ready to work!".green()
        );
        show_pull_summary();
    }
}

// 🚀 Check if working tree is clean
fn is_clean_working_tree() -> bool {
    let unstaged = Command::new("git")
        .arg("diff")
        .arg("--quiet")
        .status()
        .expect("Failed to check unstaged changes")
        .success();

    let staged = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .status()
        .expect("Failed to check staged changes")
        .success();

    unstaged && staged
}

// 🚀 Auto stash changes
fn stash_changes() {
    Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("-m")
        .arg("auto-stash-before-git-ai-pull")
        .status()
        .expect("Failed to git stash");
}

// 🚀 Normal pull
fn run_pull_normal() {
    println!("{}", "🚀 Running git pull...".cyan());
    Command::new("git")
        .arg("pull")
        .status()
        .expect("Failed to run git pull");
}

// 🚀 Pull with rebase
fn run_pull_rebase() {
    println!("{}", "🚀 Running git pull --rebase...".cyan());
    Command::new("git")
        .arg("pull")
        .arg("--rebase")
        .status()
        .expect("Failed to run git pull --rebase");
}

// 🚀 Fetch only (no merge)
fn run_git_fetch() {
    println!("{}", "🚀 Running git fetch...".cyan());
    Command::new("git")
        .arg("fetch")
        .status()
        .expect("Failed to git fetch");
}

// 🚀 Check if stash exists
fn pop_stash() {
    Command::new("git")
        .arg("stash")
        .arg("pop")
        .status()
        .expect("Failed to git stash pop");
}

// 🚀 Check if branch is behind
fn is_branch_behind() -> bool {
    let output = Command::new("git")
        .arg("status")
        .arg("-b")
        .arg("--porcelain")
        .output()
        .expect("Failed to check branch status");

    String::from_utf8_lossy(&output.stdout).contains("behind")
}

// 🚀 Show incoming commits before pulling
fn show_incoming_commits() {
    println!("{}", "\n📋 Fetching incoming commits...".cyan());

    Command::new("git")
        .arg("fetch")
        .status()
        .expect("Failed to fetch");

    let output = Command::new("git")
        .arg("log")
        .arg("HEAD..@{u}")
        .arg("--oneline")
        .output()
        .expect("Failed to get incoming commits");

    let commits = String::from_utf8_lossy(&output.stdout);

    if commits.trim().is_empty() {
        println!("✅ No new commits found.");
    } else {
        println!("🔎 Incoming commits:");
        for line in commits.lines() {
            println!("  - {}", line);
        }
    }
}

// 🚀 Decide pull strategy
fn pull_strategy_decision() {
    if detect_rebase_policy() {
        println!(
            "{}",
            "🔧 This repo prefers rebase. Running git pull --rebase...".cyan()
        );
        run_pull_rebase();
        return;
    }

    println!("\nHow would you like to pull?");
    println!("1. Pull normally");
    println!("2. Pull with rebase");
    println!("3. Fetch only (no merge yet)");
    println!("4. Abort");

    print!("Pick (1-4): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();

    match answer {
        "1" => run_pull_normal(),
        "2" => run_pull_rebase(),
        "3" => run_git_fetch(),
        "4" => {
            println!("{}", "❌ Pull aborted by user.".red());
        }
        _ => {
            println!("{}", "❌ Invalid input. Pull aborted.".red());
        }
    }
}

// 🚀 Detect if project uses rebase pull policy
fn detect_rebase_policy() -> bool {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("pull.rebase")
        .output()
        .expect("Failed to read git config pull.rebase");

    let config = String::from_utf8_lossy(&output.stdout);

    config.trim() == "true"
}

// 🚀 Check for merge conflicts
fn is_merge_conflict() -> bool {
    let output = Command::new("git")
        .arg("diff")
        .arg("--check")
        .output()
        .expect("Failed to run git diff --check");

    String::from_utf8_lossy(&output.stdout).contains("CONFLICT")
}

// Show conflict summary
fn show_conflict_summary() {
    println!("{}", "\n⚔️ Merge conflicts detected in files:".red().bold());

    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg("--diff-filter=U")
        .output()
        .expect("Failed to get conflict files");

    let files = String::from_utf8_lossy(&output.stdout);

    for file in files.lines() {
        println!("  - {}", file.bright_red());
    }
}

// Show pull summary
fn show_pull_summary() {
    println!("{}", "\n📋 Pull Summary:".bright_cyan());

    let output = Command::new("git")
        .arg("diff")
        .arg("--stat")
        .arg("HEAD@{1}")
        .arg("HEAD")
        .output()
        .expect("Failed to show pull summary");

    let summary = String::from_utf8_lossy(&output.stdout);

    if summary.trim().is_empty() {
        println!("✅ Everything is already up to date.");
    } else {
        println!("{}", summary);
    }
}
