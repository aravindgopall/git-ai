use std::env;

use crate::config::GIT_AI_CONFIG;

#[derive(Debug, Clone)]
pub enum LLMBackend {
    OpenAI,
    Azure,
    Ollama,
    Claude,
    Gemini,
    NoLLM,
}

impl LLMBackend {
    pub fn detect_backend() -> LLMBackend {
        // 1. Try from .git-ai file
        if let Some(config_backend) = &GIT_AI_CONFIG.llm_backend {
            return parse_backend(config_backend);
        }

        // 2. Try from env var
        let backend = env::var("GIT_AI_LLM").unwrap_or_else(|_| "nollm".to_string());
        parse_backend(&backend)
    }
}

fn parse_backend(backend: &str) -> LLMBackend {
    match backend.to_lowercase().as_str() {
        "openai" => {
            check_env("OPENAI_API_KEY");
            LLMBackend::OpenAI
        }
        "azure" => {
            check_env("AZURE_OPENAI_API_KEY");
            check_env("AZURE_OPENAI_API_VERSION");
            check_env("AZURE_OPENAI_ENDPOINT");
            check_env("AZURE_OPENAI_DEPLOYMENT");
            LLMBackend::Azure
        }
        "ollama" => LLMBackend::Ollama,
        "claude" => {
            check_env("CLAUDE_API_KEY");
            LLMBackend::Claude
        }
        "gemini" => {
            check_env("GEMINI_API_KEY");
            LLMBackend::Gemini
        }
        "nollm" => LLMBackend::NoLLM,
        other => {
            panic!("❌ Unknown LLM backend: '{}'", other);
        }
    }
}

fn check_env(var_name: &str) {
    if env::var(var_name).is_err() {
        panic!("❌ Required env variable '{}' not set!", var_name);
    }
}
