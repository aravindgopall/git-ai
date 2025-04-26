use crate::hunk::interactive_stage_file;
use crate::utils::{detect_language, should_ignore_file, get_combined_ignores, Language};
use colored::*;
use std::process::Command;

pub fn run_staging(interactive: bool) {
    println!("{}", "🔍 Preparing smart interactive staging...".cyan());

    let language = detect_language();
    let auto_ignores = get_combined_ignores(&language);

    match &language {
        Language::Unknown => println!("📦 Project type unknown. No auto-ignores applied."),
        _ => println!("📦 Project detected: {:?}", language),
    }

    let status_output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Failed to run git status");

    let files_changed = String::from_utf8_lossy(&status_output.stdout);

    let mut staged_files = Vec::new();
    let mut unstaged_files = Vec::new();
    let mut auto_ignored_files = Vec::new();
    let mut no_count = 0;

    for line in files_changed.lines() {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() < 3 {
            continue;
        }

        let status_x = chars[0]; // staged state
        let status_y = chars[1]; // unstaged state
        let filename = line[3..].trim();

        if should_ignore_file(filename, &auto_ignores) {
            println!("🔵 Auto-ignored: {}", filename.bright_blue());
            auto_ignored_files.push(filename.to_string());
            continue;
        }

        if status_y == 'D' {
            println!("🗑️ Deleted file detected: {}", filename.bright_red());
            unstaged_files.push(filename.to_string());
        }

        if status_x == 'M' || status_x == 'A' {
            staged_files.push(filename.to_string());
        }

        if status_y == 'M' || line.starts_with("?? ") {
            unstaged_files.push(filename.to_string());
        }
    }

    println!(
        "\n{}",
        if interactive {
            "📝 Starting Interactive Staging for unstaged files...".bright_cyan()
        } else {
            "Starting Staging for unstaged files...".bright_cyan()
        }
    );

    let mut staged_count = 0;
    for filename in unstaged_files.iter() {
        if interactive {
            interactive_stage_file(filename);
        } else {
            println!("Stage this file {}? (y/n): ", filename);
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).unwrap();

            if answer.trim().to_lowercase() == "y" {
                Command::new("git")
                    .arg("add")
                    .arg(filename)
                    .status()
                    .expect("Failed to git add");
                println!("{}", "✅ Staged!".green());
                staged_count += 1;
            } else {
                no_count += 1;
                println!("{}", "❌ Skipped!".yellow());
            }
        }
    }

    println!("\n{}", "✅ Interactive staging finished.".bright_green());

    if staged_count == 0 {
        println!("{}", "⚡ No files staged interactively.".yellow());
    } else {
        println!(
            "{}",
            format!("✅ You staged {} files during review.", staged_count).green()
        );
    }

    if unstaged_files.len() - staged_count > 1 && unstaged_files.len() - staged_count != no_count {
        println!(
            "\n🚀 {} unstaged files remain.",
            unstaged_files.len() - staged_count
        );
        println!("Do you want to auto-stage all remaining files? (y/n)");

        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().to_lowercase() == "y" {
            for file in unstaged_files.iter() {
                Command::new("git")
                    .arg("add")
                    .arg(file)
                    .status()
                    .expect("Failed to git add");
            }
            println!("{}", "🚀 Auto-staged remaining files.".bright_green());
        } else {
            println!("{}", "🛑 Left files unstaged as per your choice.".yellow());
        }
    }

    if !auto_ignored_files.is_empty() {
        println!(
            "\n{}",
            format!("🔵 Auto-ignored {} junk files!", auto_ignored_files.len()).bright_yellow()
        );
    }
}
