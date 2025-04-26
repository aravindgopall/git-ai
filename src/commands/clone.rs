use crate::config::GIT_AI_CONFIG;
use colored::*;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn smart_clone(url: String) {
    println!("{}", "🔍 Preparing to clone repository...".cyan());

    let url = maybe_switch_to_ssh(url);

    if !run_git_clone(&url) {
        println!("{}", "❌ Git clone failed.".red());
        return;
    }

    let repo_name = extract_repo_name(&url);

    println!("✅ Repo cloned into {}/", repo_name);

    if Path::new(&repo_name).exists() {
        post_clone_scan(&repo_name);
        open_in_editor(&repo_name);
    } else {
        println!(
            "{}",
            "⚠️ Repo directory not found after clone. Please open manually.".yellow()
        );
    }
}

// Offer to switch HTTPS → SSH
fn maybe_switch_to_ssh(url: String) -> String {
    if url.starts_with("https://github.com/") {
        println!("⚡ You are cloning over HTTPS.");
        println!("🛡️ Would you prefer to clone using SSH instead? (y/n)");

        let mut answer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim().to_lowercase();

        if answer == "y" {
            let ssh_url = url
                .replace("https://github.com/", "git@github.com:")
                .replace(".git", "")
                + ".git";
            println!("✅ Switching to SSH: {}", ssh_url.bright_cyan());
            ensure_ssh_agent();
            return ssh_url;
        }
    }
    url
}

// Auto-start ssh-agent if needed
fn ensure_ssh_agent() {
    if std::env::var("SSH_AUTH_SOCK").is_ok() {
        println!("🔑 SSH agent already running.");
    } else {
        println!("🔑 Starting SSH agent...");
        Command::new("ssh-agent")
            .arg("-s")
            .status()
            .expect("Failed to start ssh-agent");

        Command::new("ssh-add")
            .status()
            .expect("Failed to add SSH key");
    }
}

// Run git clone
fn run_git_clone(url: &str) -> bool {
    Command::new("git")
        .arg("clone")
        .arg(url)
        .status()
        .expect("Failed to run git clone")
        .success()
}

// Get repo name
fn extract_repo_name(url: &str) -> String {
    url.split('/').last().unwrap_or("repo").replace(".git", "")
}

// Scan project after clone
fn post_clone_scan(repo: &str) {
    println!("{}", "\n🚀 Quick scan after clone...".bright_cyan());

    let repo_path = Path::new(repo);

    if !repo_path.join(".gitignore").exists() {
        println!("⚡ No .gitignore found.");
        println!("🛡️ Would you like to generate one with `git-ai init`? (y/n)");

        let mut answer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim().to_lowercase();

        if answer == "y" {
            println!("✨ Running git-ai init...");
            // Ideally, spawn `git-ai init` here but for now hint the user
            println!("👉 Inside repo, run: `git-ai init` 🚀");
        }
    }

    if repo_path.join("apps").exists() || repo_path.join("packages").exists() {
        println!("⚡ Detected Monorepo structure (apps/, packages/).");
    }

    if !repo_path.join("README.md").exists() {
        println!("⚡ No README.md found.");
    }
}

// Try opening editor
fn open_in_editor(repo: &str) {
    println!("📦 Trying to open {} in your editor...", repo);

    if let Ok(editor) = std::env::var("EDITOR") {
        println!("🛠️ Using editor from $EDITOR: {}", editor);
        Command::new(editor)
            .arg(".")
            .current_dir(repo)
            .status()
            .expect("Failed to open editor");
        return;
    }
    if let Some(editor) = &GIT_AI_CONFIG.editor {
        println!("🛠️ Using editor from $EDITOR: {}", editor);
        Command::new(editor)
            .arg(".")
            .current_dir(repo)
            .status()
            .expect("Failed to open editor");
        return;
    }

    if check_command_exists("code") {
        println!("🛠️ Opening with VSCode...");
        Command::new("code")
            .arg(".")
            .current_dir(repo)
            .status()
            .expect("Failed to open VSCode");
        return;
    }

    if check_command_exists("nvim") {
        println!("🛠️ Opening with Neovim...");
        Command::new("nvim")
            .arg(".")
            .current_dir(repo)
            .status()
            .expect("Failed to open Neovim");
        return;
    }

    if check_command_exists("vim") {
        println!("🛠️ Opening with Vim...");
        Command::new("vim")
            .arg(".")
            .current_dir(repo)
            .status()
            .expect("Failed to open Vim");
        return;
    }

    println!("⚠️ No known editor detected. Please open manually.");
}

// Check if a command exists
fn check_command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
