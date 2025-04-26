use crate::utils::{detect_language, get_combined_ignores, should_ignore_file};
use colored::*;
use std::fs;
use std::process::Command;
use std::time::{Duration, SystemTime};

pub fn show_git_status() {
    println!("{}", "🔍 Checking Git Status...".cyan());

    // Detect language and auto-ignore patterns
    let language = detect_language();
    let auto_ignores = get_combined_ignores(&language);

    let output = Command::new("git")
        .arg("status")
        .arg("--short")
        .output()
        .expect("Failed to run git status");

    let status_text = String::from_utf8_lossy(&output.stdout);

    if status_text.trim().is_empty() {
        println!("{}", "✅ Working tree clean. No changes.".green());
        suggest_hint_clean_repo();
        return;
    }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    let mut deleted = Vec::new();
    let mut new_files = Vec::new();
    let mut others = Vec::new();

    for line in status_text.lines() {
        let status_x = line.chars().nth(0).unwrap_or(' ');
        let status_y = line.chars().nth(1).unwrap_or(' ');
        let filename = &line[3..];

        // 💥 Ignore files matching patterns
        if should_ignore_file(filename, &auto_ignores) {
            continue;
        }

        match (status_x, status_y) {
            ('M', _) | (_, 'M') => modified.push(filename.to_string()),
            ('A', _) | (_, 'A') => added.push(filename.to_string()),
            ('D', _) | (_, 'D') => deleted.push(filename.to_string()),
            ('?', '?') => new_files.push(filename.to_string()),
            _ => others.push(filename.to_string()),
        }
    }

    if !modified.is_empty() {
        println!("\n📝 Modified files:");
        for file in &modified {
            println!("  - {} {}", file.bright_blue(), get_modified_time(file));
        }
    }

    if !added.is_empty() {
        println!("\n✨ Added files:");
        for file in &added {
            println!("  - {} {}", file.bright_green(), get_modified_time(file));
        }
    }

    if !deleted.is_empty() {
        println!("\n🗑️ Deleted files:");
        for file in &deleted {
            println!("  - {}", file.bright_red());
        }
    }

    if !new_files.is_empty() {
        println!("\n🆕 New (untracked) files:");
        for file in &new_files {
            println!("  - {} {}", file.bright_yellow(), get_modified_time(file));
        }
    }

    if !others.is_empty() {
        println!("\n❔ Other changes:");
        for file in &others {
            println!("  - {}", file.normal());
        }
    }

    show_branch_info();
    show_merge_conflict_detection();
}

// 🕰 Get last modified time for a file
fn get_modified_time(file_path: &str) -> String {
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = SystemTime::now().duration_since(modified) {
                return format!("(modified {})", format_duration(duration));
            }
        }
    }
    "".to_string()
}

// 🕰 Format duration nicely
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{} seconds ago", secs)
    } else if secs < 3600 {
        format!("{} minutes ago", secs / 60)
    } else if secs < 86400 {
        format!("{} hours ago", secs / 3600)
    } else {
        format!("{} days ago", secs / 86400)
    }
}

// 📦 Branch info + detached detection
fn show_branch_info() {
    println!("\n🔎 Checking branch info...");

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to get branch name");

    let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if branch_name == "HEAD" {
        println!(
            "{}",
            "⚠️ Detached HEAD detected! You are not on any branch."
                .red()
                .bold()
        );
        return;
    }

    // Compare with remote
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--left-right")
        .arg("--count")
        .arg(format!("origin/{}...{}", branch_name, branch_name))
        .output()
        .expect("Failed to check branch sync");

    let result = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = result.trim().split_whitespace().collect();

    if parts.len() == 2 {
        let behind: u32 = parts[0].parse().unwrap_or(0);
        let ahead: u32 = parts[1].parse().unwrap_or(0);

        if behind == 0 && ahead == 0 {
            println!(
                "📦 Branch: {} (✅ Up to date with origin)",
                branch_name.bright_green()
            );
        } else {
            println!("📦 Branch: {}", branch_name.bright_magenta());
            if ahead > 0 {
                println!("🚀 Ahead of remote by {} commits", ahead);
            }
            if behind > 0 {
                println!("📥 Behind remote by {} commits", behind);
            }
        }
    } else {
        println!("📦 Branch: {}", branch_name.bright_magenta());
    }
}

// 🛡️ Merge conflict detection
fn show_merge_conflict_detection() {
    println!("\n🔎 Checking for merge conflicts...");

    let output = Command::new("git")
        .arg("diff")
        .arg("--check")
        .output()
        .expect("Failed to run git diff --check");

    let status_text = String::from_utf8_lossy(&output.stdout);

    if status_text.contains("CONFLICT") {
        println!("{}", "❗ Merge conflicts detected!".red().bold());
    } else {
        println!("{}", "✅ No merge conflicts detected.".green());
    }
}

fn suggest_hint_clean_repo() {
    println!(
        "\n🚀 Hint: Working tree clean. You can safely pull latest changes or start new work!"
    );
}
