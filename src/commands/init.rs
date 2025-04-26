use colored::*;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::process::Command;

use crate::ai::generate_project_scaffolding;

pub async fn smart_init(magic: bool) {
    println!("{}", "📦 Initializing Git repository...".cyan());
    if magic {
        magic_init().await;
        return;
    }
    normal_init()
}

async fn magic_init() {
    println!("{}", "🔮 Performing Magic Init using AI...".bright_cyan());

    if Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to run git init")
        .success()
    {
        println!("{}", "✅ Git repository created!".green());
    } else {
        println!("{}", "❌ Failed to initialize Git.".red());
        return;
    }

    let file_list = scan_project_files();

    if file_list.is_empty() {
        println!(
            "{}",
            "❌ No files found to analyze. Falling back to normal init.".red()
        );
        normal_init();
        return;
    }

    println!("📋 Found {} files to analyze.", file_list.len());

    let system_prompt = "You are a Git repository initializer.\n\nGiven the following project files, Please:\n1. Guess the project type (e.g., Rust, Node.js, Python, etc.)\n2. Suggest a clean .gitignore\n3. Suggest a clean .git-ai-ignore\n4. Suggest a starter README.md\n\nReturn each section clearly titled.";
    let files_input = build_magic_init_files(&file_list);

    println!(
        "{}",
        "🤖 Asking AI to generate project scaffolding...".cyan()
    );

    match generate_project_scaffolding(&system_prompt, &files_input).await {
        Ok(ai_response) => {
            println!(
                "{}",
                "✨ AI Suggested Project Scaffolding:\n".bright_magenta()
            );
            println!("{}", ai_response.bright_white());

            println!(
                "{}",
                "\n✅ Do you want to apply this magic setup? (y/n)".bright_cyan()
            );
            let mut answer = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut answer).unwrap();
            let answer = answer.trim().to_lowercase();

            if answer == "y" {
                save_magic_scaffolding(&ai_response);
            } else {
                println!("{}", "❌ Magic init cancelled by user.".red());
            }
        }
        Err(_) => {
            println!(
                "{}",
                "❌ AI failed to generate project scaffolding. Falling back to normal init.".red()
            );
            normal_init();
        }
    }
}

// Scan project files
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

// Build magic LLM files
fn build_magic_init_files(files: &[String]) -> String {
    let preview = files
        .iter()
        .take(50)
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    format!("\ngenerate for these project files: {}", preview)
}

// Save AI generated files
fn save_magic_scaffolding(ai_response: &str) {
    let sections: Vec<&str> = ai_response.split("###").collect();

    for section in sections {
        let trimmed = section.trim().to_lowercase();

        if trimmed.contains(".gitignore") {
            save_file_from_section(section, ".gitignore");
        } else if trimmed.contains(".git-ai-ignore") {
            save_file_from_section(section, ".git-ai-ignore");
        } else if trimmed.contains("readme.md") {
            save_file_from_section(section, "README.md");
        }
    }
}

fn save_file_from_section(content: &str, filename: &str) {
    println!("💾 Saving {}...", filename);

    let cleaned = content
        .lines()
        .skip(1) // Skip title line like "### .gitignore"
        .collect::<Vec<&str>>()
        .join("\n");

    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(cleaned.as_bytes())
        .expect("Failed to write file");

    println!("✅ {} created!", filename);
}

pub fn normal_init() {
    if Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to run git init")
        .success()
    {
        println!("{}", "✅ Git repository created!".green());
    } else {
        println!("{}", "❌ Failed to initialize Git.".red());
        return;
    }

    setup_gitignore();
    setup_git_ai_ignore();
    setup_readme();
    setup_git_config();

    println!("{}", "🚀 All ready! Start building!".bright_cyan());
}

// Setup .gitignore
fn setup_gitignore() {
    if fs::metadata(".gitignore").is_ok() {
        println!("⚡ .gitignore already exists, skipping.");
        return;
    }

    let mut file = File::create(".gitignore").expect("Failed to create .gitignore");

    writeln!(
        file,
        "# Standard Git ignores\n/target/\n/node_modules/\n/dist/\n.env\n*.log\n.DS_Store"
    )
    .expect("Failed to write to .gitignore");

    println!("{}", "✅ .gitignore created.".green());
}

// Setup .git-ai-ignore
fn setup_git_ai_ignore() {
    if fs::metadata(".git-ai-ignore").is_ok() {
        println!("⚡ .git-ai-ignore already exists, skipping.");
        return;
    }

    let mut file = File::create(".git-ai-ignore").expect("Failed to create .git-ai-ignore");

    writeln!(
        file,
        "# Git-AI ignores\n/target/\n/node_modules/\n/build/\n*.tmp\n*.cache"
    )
    .expect("Failed to write to .git-ai-ignore");

    println!("{}", "✅ .git-ai-ignore created.".green());
}

// Setup starter README.md
fn setup_readme() {
    if fs::metadata("README.md").is_ok() {
        println!("⚡ README.md already exists, skipping.");
        return;
    }

    let mut file = File::create("README.md").expect("Failed to create README.md");

    writeln!(
        file,
        "# New Git-AI Project\n\nGenerated with `git-ai init` 🚀\n"
    )
    .expect("Failed to write to README.md");

    println!("{}", "✅ README.md created.".green());
}

// Optional: Setup Git user configs if not set
fn setup_git_config() {
    let name_output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()
        .expect("Failed to check git user.name");

    let email_output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .output()
        .expect("Failed to check git user.email");

    let name = String::from_utf8_lossy(&name_output.stdout)
        .trim()
        .to_string();
    let email = String::from_utf8_lossy(&email_output.stdout)
        .trim()
        .to_string();

    if name.is_empty() {
        println!("{}", "🛠️ Git user.name not set. Set it now:".cyan());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            Command::new("git")
                .arg("config")
                .arg("user.name")
                .arg(input)
                .status()
                .expect("Failed to set git user.name");
            println!("✅ user.name set.");
        }
    }

    if email.is_empty() {
        println!("{}", "🛠️ Git user.email not set. Set it now:".cyan());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            Command::new("git")
                .arg("config")
                .arg("user.email")
                .arg(input)
                .status()
                .expect("Failed to set git user.email");
            println!("✅ user.email set.");
        }
    }
}
