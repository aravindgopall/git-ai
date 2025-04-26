use colored::*;
use glob::glob;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use terminal_size::{terminal_size, Height};

#[derive(Debug, Clone)]
pub enum Language {
    Rust,
    Node,
    Python,
    Java,
    Haskell,
    Unknown,
}

pub fn detect_language() -> Language {
    if Path::new("Cargo.toml").exists() {
        Language::Rust
    } else if Path::new("package.json").exists() {
        Language::Node
    } else if Path::new("requirements.txt").exists() || Path::new("setup.py").exists() {
        Language::Python
    } else if Path::new("pom.xml").exists() {
        Language::Java
    } else if has_cabal_file() {
        Language::Haskell
    } else {
        Language::Unknown
    }
}

fn has_cabal_file() -> bool {
    glob("*.cabal")
        .expect("Failed to read glob pattern")
        .any(|entry| entry.is_ok())
}

pub fn get_auto_ignores(language: &Language) -> Vec<String> {
    match language {
        Language::Rust => vec!["target/".to_string()],
        Language::Node => vec!["node_modules/".to_string(), "dist/".to_string()],
        Language::Python => vec!["__pycache__/".to_string(), ".pyc".to_string()],
        Language::Java => vec!["target/".to_string(), ".class".to_string()],
        Language::Haskell => vec!["dist-newstyle/".to_string(), ".stack-work/".to_string()],
        Language::Unknown => vec![],
    }
}

// backward compatability
fn load_git_ignore() -> Vec<String> {
    if let Ok(content) = fs::read_to_string(".gitignore") {
        content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect()
    } else {
        vec![]
    }
}

fn load_git_ai_ignore() -> Vec<String> {
    if let Ok(content) = fs::read_to_string(".git-ai-ignore") {
        content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect()
    } else {
        vec![]
    }
}

pub fn get_combined_ignores(language: &Language) -> Vec<String> {
    let mut combined = get_auto_ignores(language);
    combined.extend(load_git_ai_ignore());
    combined.extend(load_git_ignore());
    combined
}

// Check if a file should be ignored
pub fn should_ignore_file(filename: &str, ignore_patterns: &[String]) -> bool {
    ignore_patterns.iter().any(|pat| filename.contains(pat))
}

fn get_terminal_height() -> usize {
    if let Some((_, Height(h))) = terminal_size() {
        (h as usize).saturating_sub(6)
    } else {
        20 // fallback if detection fails
    }
}

pub fn show_in_pager(content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let lines_per_page = get_terminal_height();

    let mut current_line = 0;

    while current_line < total_lines {
        let end_line = usize::min(current_line + lines_per_page, total_lines);

        for line in &lines[current_line..end_line] {
            println!("{}", line);
        }

        if end_line >= total_lines {
            break;
        }

        print!("{}", "-- More -- (press enter to continue, q to quit) ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 🛠 Clear the "-- More --" line
        print!("\x1b[F\x1b[2K");
        io::stdout().flush().unwrap();

        match input.trim() {
            " " => {
                current_line = end_line; // ✅ Move to next page correctly
            }
            "q" | "Q" => {
                println!("{}", "\n🛑 Quit paging.".red());
                break;
            }
            _ => {
                current_line = end_line;
            }
        }
    }
}
