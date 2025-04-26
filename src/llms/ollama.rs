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
        let client = Client::new();

        let body = json!({
            "model": "mistral", // Default local model (user can customize later)
            "prompt": format!("Generate a Git commit message for this diff:\n{}", diff),
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
}
