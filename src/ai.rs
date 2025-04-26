use crate::llms::LLMProvider;
use crate::llms::{self, backend::LLMBackend};
use once_cell::sync::Lazy;
use rand::prelude::*;
use std::error::Error;
use std::sync::Mutex;

pub static BACKEND: Lazy<Mutex<Option<LLMBackend>>> = Lazy::new(|| Mutex::new(None));

pub fn init_llm_backend() {
    let backend = LLMBackend::detect_backend();
    let mut global_backend = BACKEND.lock().unwrap();
    *global_backend = Some(backend);
}

pub async fn generate_commit_message(
    diff: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let backend = BACKEND.lock().unwrap();
    let backend = backend.clone().expect("Backend not initialized!");

    match backend {
        LLMBackend::OpenAI => llms::openai::OpenAIProvider::generate_commit_message(diff).await,
        LLMBackend::Azure => llms::azure::AzureOpenAIProvider::generate_commit_message(diff).await,
        LLMBackend::Ollama => llms::ollama::OllamaProvider::generate_commit_message(diff).await,
        LLMBackend::Claude => llms::claude::ClaudeProvider::generate_commit_message(diff).await,
        LLMBackend::Gemini => llms::gemini::GeminiProvider::generate_commit_message(diff).await,
        LLMBackend::NoLLM => Err(Box::<dyn Error + Send + Sync>::from("no_llm is set")),
    }
}
pub async fn generate_project_scaffolding(
    prompt: &str,
    input: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let backend = BACKEND.lock().unwrap();
    let backend = backend.clone().expect("Backend not initialized!");

    match backend {
        LLMBackend::OpenAI => {
            llms::openai::OpenAIProvider::generate_project_scaffolding(prompt, input).await
        }
        LLMBackend::Azure => {
            llms::azure::AzureOpenAIProvider::generate_project_scaffolding(prompt, input).await
        }
        LLMBackend::Ollama => {
            llms::ollama::OllamaProvider::generate_project_scaffolding(prompt, input).await
        }
        LLMBackend::Claude => {
            llms::claude::ClaudeProvider::generate_project_scaffolding(prompt, input).await
        }
        LLMBackend::Gemini => {
            llms::gemini::GeminiProvider::generate_project_scaffolding(prompt, input).await
        }
        LLMBackend::NoLLM => Err(Box::<dyn Error + Send + Sync>::from("no_llm is set")),
    }
}

pub fn suggest_commit_message() -> String {
    let options = vec![
        "[git-ai] cleaned config spaghetti 🍝",
        "[git-ai] squashed timestamps like a boss 🕒",
        "[git-ai] localhost chaos managed 🚀",
        "[git-ai] peace restored to your diffs ✌️",
        "[git-ai] tiny tweaks, big vibes 🎯",
        "[git-ai] silenced noisy paths 🔇",
    ];

    options.choose(&mut rand::rng()).unwrap().to_string()
}
