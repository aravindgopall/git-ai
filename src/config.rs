use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use toml;

pub static GIT_AI_CONFIG: Lazy<GitAIConfig> = Lazy::new(|| load_git_ai_config());

#[derive(Debug, Deserialize, Clone)]
pub struct GitAIConfig {
    pub auto_commit: Option<bool>,
    pub auto_push: Option<bool>,
    pub editor: Option<String>,
    pub llm_backend: Option<String>,
    pub ai_enabled: Option<bool>,
}

impl Default for GitAIConfig {
    fn default() -> Self {
        GitAIConfig {
            auto_commit: Some(false),
            auto_push: Some(false),
            editor: None,
            llm_backend: None,
            ai_enabled: Some(false),
        }
    }
}

fn load_git_ai_config() -> GitAIConfig {
    if let Ok(content) = fs::read_to_string(".git-ai") {
        toml::from_str(&content).unwrap_or_default()
    } else {
        GitAIConfig::default()
    }
}

pub fn load_profile(profile_name: String) -> Vec<String> {
    let config_content = fs::read_to_string(".git-ai-config").unwrap_or_default();

    let mut ignores = Vec::new();
    let lines: Vec<&str> = config_content.lines().collect();

    let mut in_profile = false;
    for line in lines {
        if line.trim() == format!("[{}]", profile_name) {
            in_profile = true;
            continue;
        }
        if in_profile {
            if line.starts_with('[') {
                break;
            }
            ignores.push(line.trim().to_string());
        }
    }

    ignores
}
