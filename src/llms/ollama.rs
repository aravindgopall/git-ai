use super::LLMProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OllamaProvider;

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        call(
            diff,
            "You are a Git commit message generator. Write clear, concise Git commit messages.",
        )
        .await
    }
    async fn generate_project_scaffolding(
        prompt: &str,
        input: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        call(input, prompt).await
    }
}

async fn call(
    input: &str,
    system_prompt: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let body = json!({
        "model": "mistral", // Default local model (user can customize later)
        "prompt": format!("{}\n {}",system_prompt, input),
        "stream": false
    });

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;

    let message = json["response"]
        .as_str()
        .unwrap_or("Generated commit message")
        .to_string();

    Ok(message.trim().to_string())
}
