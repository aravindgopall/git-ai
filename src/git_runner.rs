use crate::{ai, config, filters, interact, prompts};
use colored::*;
use std::process::Command;

pub fn run_diff(prompt: Option<String>, profile: Option<String>, interactive: bool) {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to run git diff");

    let diff_text = String::from_utf8_lossy(&output.stdout);

    let mut ignores = vec![];

    if let Some(p) = prompt {
        ignores.extend(prompts::parse_prompt_to_ignores(&p));
    }

    if let Some(profile_name) = profile {
        ignores.extend(config::load_profile(profile_name));
    }

    let cleaned_diff = filters::apply_ignores(diff_text.to_string(), ignores);

    if interactive {
        interact::start_interactive_review(cleaned_diff);
    } else {
        println!("{}", cleaned_diff.bright_white());
    }
    let funny_commit = ai::suggest_commit_message();
    println!(
        "\n✨ Suggested Commit Message: {}",
        funny_commit.bright_magenta()
    );
    println!("Use this commit message? (y/n)");

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().to_lowercase() == "y" {
        std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(funny_commit)
            .status()
            .expect("Failed to git commit");
        println!("{}", "✅ Committed!".green());
    } else {
        println!(
            "{}",
            "❌ Skipping auto-commit. You can commit manually.".yellow()
        );
    }
}
