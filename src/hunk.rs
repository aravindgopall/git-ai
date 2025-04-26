use colored::*;
use std::process::{Command, Stdio};
use std::io::{self, Write};

use crate::utils::show_in_pager;

pub fn interactive_stage_file(filename: &str) {
    println!("📝 Building full hunks for file: {}", filename.bright_blue());

    let diff_output = Command::new("git")
        .arg("diff")
        .arg(filename)
        .output()
        .expect("Failed to run git diff");

    let diff_text = String::from_utf8_lossy(&diff_output.stdout);

    if diff_text.trim().is_empty() {
        println!("{}", "No changes found to stage.".yellow());
        return;
    }

    let (header, hunks) = split_diff_into_hunks(&diff_text);

    if hunks.is_empty() {
        println!("{}", "No hunks found.".yellow());
        return;
    }

    let mut stage_all = false;
    let mut ignore_all = false;
    for hunk in hunks {
        if stage_all {
            apply_hunk(filename, &header, &hunk);
            continue;
        }
        if ignore_all {
            break;
        }

        show_in_pager(&hunk);  
        println!("Stage this hunk? (y = yes, n = no, Y = yes all, N = no all, q = quit): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim();

        match answer {
            "y" => {
                apply_hunk(filename, &header, &hunk);
                println!("{}", "✅ Hunk staged.".green());
            }
            "n" => {
                println!("{}", "❌ Skipped.".yellow());
            }
            "N" => {
                ignore_all = true;
                println!("{}", "❌ Skipped all files.".yellow());
            }
            "Y" => {
                apply_hunk(filename, &header, &hunk);
                stage_all = true;
                println!("{}", "✅ Staging all remaining hunks.".green());
            }
            "q" | "Q" => {
                println!("{}", "🛑 Exiting hunk staging.".red());
                std::process::exit(0);
            }
            _ => {
                println!("{}", "Invalid choice. Skipping hunk.".yellow());
            }
        }
    }
}

fn split_diff_into_hunks(diff_text: &str) -> (String, Vec<String>) {
    let mut header = String::new();
    let mut hunks = Vec::new();
    let mut current_hunk = String::new();
    let mut in_hunk = false;

    for line in diff_text.lines() {
        if line.starts_with("diff --git") {
            header.push_str(line);
            header.push('\n');
        } else if line.starts_with("index ") || line.starts_with("--- ") || line.starts_with("+++ ") {
            header.push_str(line);
            header.push('\n');
        } else if line.starts_with("@@") {
            if in_hunk {
                hunks.push(current_hunk.clone());
                current_hunk.clear();
            }
            in_hunk = true;
            current_hunk.push_str(line);
            current_hunk.push('\n');
        } else if in_hunk {
            current_hunk.push_str(line);
            current_hunk.push('\n');
        }
    }

    if !current_hunk.is_empty() {
        hunks.push(current_hunk);
    }

    (header, hunks)
}

fn apply_hunk(_filename: &str, header: &str, hunk_text: &str) {
    let patch_content = format!("{}{}", header, hunk_text);

    let mut patch_cmd = Command::new("git")
        .arg("apply")
        .arg("--cached")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run git apply");

    {
        let stdin = patch_cmd.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(patch_content.as_bytes())
            .expect("Failed to write patch content");
    }

    patch_cmd.wait().expect("Failed to wait on git apply");
}

