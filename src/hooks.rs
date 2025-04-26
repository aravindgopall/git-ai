use crate::utils::{detect_language, get_auto_ignores};
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn install_hook() {
    let hook_path = ".git/hooks/pre-commit";
    if Path::new(hook_path).exists() {
        println!("Hook already exists. Overwrite manually if needed.");
        return;
    }

    let script = r#"#!/bin/sh
git-ai precommit
"#;
    fs::write(hook_path, script).expect("Failed to write hook file");
    Command::new("chmod")
        .arg("+x")
        .arg(hook_path)
        .status()
        .expect("Failed to chmod hook");
    println!("✅ Pre-commit hook installed!");
}

pub fn uninstall_hook() {
    let hook_path = ".git/hooks/pre-commit";
    if Path::new(hook_path).exists() {
        fs::remove_file(hook_path).expect("Failed to remove hook");
        println!("🗑️ Hook uninstalled.");
    } else {
        println!("No hook to uninstall.");
    }
}

pub fn run_precommit() {
    println!("{}", "🔒 git-ai precommit check starting...".cyan());

    let language = detect_language();
    let auto_ignores = get_auto_ignores(&language);

    if auto_ignores.is_empty() {
        println!("No auto-ignores detected for this project type.");
        return;
    }

    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--name-only")
        .output()
        .expect("Failed to check staged files");

    let staged_files = String::from_utf8_lossy(&output.stdout);

    let mut junk_detected = vec![];

    for file in staged_files.lines() {
        if auto_ignores.iter().any(|junk| file.contains(junk)) {
            junk_detected.push(file.to_string());
        }
    }

    if junk_detected.is_empty() {
        println!("{}", "✅ No junk files staged. Good to go!".green());
        return;
    }

    println!("{}", "⚠️  Warning: Junk files staged!".yellow());
    for file in junk_detected.iter() {
        println!("🔵 {}", file.bright_yellow());
    }

    println!("Unstage junk files automatically? (y/n)");

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().to_lowercase() == "y" {
        for file in junk_detected {
            Command::new("git")
                .arg("reset")
                .arg("HEAD")
                .arg(&file)
                .status()
                .expect("Failed to unstage file");
            println!("✅ Unstaged {}", file);
        }
        println!("{}", "🚀 Cleaned junk files. Continue with commit.".green());
    } else {
        println!("{}", "⚠️  Continuing, but junk files still staged.".red());
    }
}
