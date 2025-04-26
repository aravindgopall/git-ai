use crate::ai::generate_commit_message;
use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub async fn ignore_handler(suggest: bool, save: bool) {
    if suggest {
        suggest_ignore_patterns(save).await;
    } else {
        println!("{}", "❌ No action specified for git-ai ignore.".red());
    }
}

// Suggest ignore patterns using AI
async fn suggest_ignore_patterns(save: bool) {
    println!("{}", "🔍 Scanning project files...".cyan());

    let file_list = scan_project_files();

    if file_list.is_empty() {
        println!("{}", "❌ No files found to scan.".red());
        return;
    }

    println!("📋 Found {} files to analyze.", file_list.len());

    let prompt = build_ignore_prompt(&file_list);

    println!("{}", "🤖 Asking AI to suggest ignore patterns...".cyan());

    match generate_commit_message(&prompt).await {
        Ok(suggestions) => {
            println!(
                "\n✨ AI Suggested Ignore Patterns:\n\n{}",
                suggestions.bright_magenta()
            );

            if save {
                save_ignore_file(&suggestions);
            } else {
                println!(
                    "{}",
                    "\n🛡️ Review these suggestions carefully before using.".cyan()
                );
            }
        }
        Err(_) => {
            println!("{}", "❌ Failed to generate ignore suggestions.".red());
        }
    }
}

// Scan project files (non-recursive for now)
fn scan_project_files() -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() || file_type.is_dir() {
                    if let Ok(path) = entry.path().into_os_string().into_string() {
                        files.push(path);
                    }
                }
            }
        }
    }

    files
}

// Build prompt for LLM
fn build_ignore_prompt(files: &[String]) -> String {
    let preview = files
        .iter()
        .take(50)
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "Based on these project files:\n{}\n\nSuggest a .gitignore-style list of patterns that should be ignored to keep Git history clean. Focus on junk files, build artifacts, temporary files, environment files, node_modules, target, etc.",
        preview
    )
}

fn save_ignore_file(content: &str) {
    println!("{}", "💾 Saving suggestions into .git-ai-ignore...".cyan());

    let mut file = OpenOptions::new()
        .create(true) // create if not exists
        .append(true) // append if exists
        .open(".git-ai-ignore")
        .expect("Failed to open or create .git-ai-ignore");

    let mut gitfile = OpenOptions::new()
        .create(true) // create if not exists
        .append(true) // append if exists
        .open(".gitignore")
        .expect("Failed to open or create .gitagnore");

    writeln!(file, "\n# Added by git-ai\n{}", content).expect("Failed to write to .git-ai-ignore");
    writeln!(gitfile, "\n# Added by git-ai\n{}", content).expect("Failed to write to .gitignore");
    println!(
        "{}",
        "✅ Appended ignore patterns into .git-ai-ignore, .gitignore".green()
    );
}
