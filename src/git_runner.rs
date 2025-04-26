use crate::{config, filters, interact, prompts};
use colored::*;
use std::process::Command;

pub fn run_diff(prompt: Option<String>, profile: Option<String>) {
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

    interact::start_interactive_review(cleaned_diff);
}
