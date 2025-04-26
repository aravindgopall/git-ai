use colored::*;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::ai::generate_project_scaffolding;

#[derive(Debug, Clone)]
pub enum ProjectLanguage {
    Rust,
    Node,
    Python,
    Java,
    Go,
    Haskell,
    Unknown,
}

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

fn interactive_select_language() -> ProjectLanguage {
    println!("\n⚡ Could not auto-detect project type.");
    println!("What language is this project in?");
    println!("1. Rust");
    println!("2. Node.js");
    println!("3. Python");
    println!("4. Java");
    println!("5. Go");
    println!("6. Haskell");
    println!("7. Other");

    print!("Pick (1-7): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => ProjectLanguage::Rust,
        "2" => ProjectLanguage::Node,
        "3" => ProjectLanguage::Python,
        "4" => ProjectLanguage::Java,
        "5" => ProjectLanguage::Go,
        "6" => ProjectLanguage::Haskell,
        _ => ProjectLanguage::Unknown,
    }
}

pub fn normal_init() {
    let language = interactive_select_language();
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
    setup_gitignore(&language);
    setup_git_ai_ignore(&language);
    setup_readme();
    setup_git_config();

    println!("{}", "🚀 All ready! Start building!".bright_cyan());
}

// Append to existing .gitignore
fn append_gitignore(language: &ProjectLanguage) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(".gitignore")
        .expect("Failed to open .gitignore");
    write_gitignore_contents(&mut file, language);
}

// Append to existing .git-ai-ignore
fn append_gitai_ignore(language: &ProjectLanguage) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(".git-ai-ignore")
        .expect("Failed to open .git-ai-ignore");
    write_gitai_ignore_contents(&mut file, language);
}

// Content writing for .gitignore
fn write_gitignore_contents(file: &mut File, language: &ProjectLanguage) {
    let patterns = match language {
        ProjectLanguage::Rust => "/target/\nCargo.lock",
        ProjectLanguage::Node => "/node_modules/\n/dist/\n.env",
        ProjectLanguage::Python => "__pycache__/\n*.pyc\n.venv/",
        ProjectLanguage::Java => "target/\n*.class",
        ProjectLanguage::Go => "bin/\npkg/\n*.exe",
        ProjectLanguage::Haskell => "dist/\n*.hi\n*.o",
        ProjectLanguage::Unknown => "*.log\n*.tmp\n.DS_Store",
    };

    writeln!(file, "\n# Added by git-ai\n{}", patterns).expect("Failed to write to file");
}

// Content writing for .git-ai-ignore
fn write_gitai_ignore_contents(file: &mut File, language: &ProjectLanguage) {
    let patterns = match language {
        ProjectLanguage::Rust => "/target/\nCargo.lock",
        ProjectLanguage::Node => "/node_modules/\n/dist/\n.env\n*.lock",
        ProjectLanguage::Python => "__pycache__/\n*.pyc\n*.pyo\n.env",
        ProjectLanguage::Java => "/target/\n*.class\n.env",
        ProjectLanguage::Go => "/bin/\n/pkg/\n*.exe\n*.test",
        ProjectLanguage::Haskell => "/dist/\n*.hi\n*.o",
        ProjectLanguage::Unknown => "*.log\n*.tmp\n*.cache",
    };

    writeln!(file, "\n# Added by git-ai\n{}", patterns).expect("Failed to write to file");
}

fn setup_gitignore(language: &ProjectLanguage) {
    if Path::new(".gitignore").exists() {
        println!("⚡ .gitignore already exists, appending...");
        append_gitignore(language);
        return;
    }

    let mut file = File::create(".gitignore").expect("Failed to create .gitignore");
    write_gitignore_contents(&mut file, language);
    println!("{}", "✅ .gitignore created.".green());
}

// Setup .git-ai-ignore
fn setup_git_ai_ignore(language: &ProjectLanguage) {
    if Path::new(".git-ai-ignore").exists() {
        println!("⚡ .git-ai-ignore already exists, appending...");
        append_gitai_ignore(language);
        return;
    }

    let mut file = File::create(".git-ai-ignore").expect("Failed to create .git-ai-ignore");
    write_gitai_ignore_contents(&mut file, language);
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
