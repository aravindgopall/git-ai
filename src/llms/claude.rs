use super::LLMProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct ClaudeProvider;

#[async_trait]
impl LLMProvider for ClaudeProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let api_key = std::env::var("CLAUDE_API_KEY").expect("CLAUDE_API_KEY not set");

        let body = json!({
            "model": "claude-3-opus-20240229",
            "messages": [
                {"role": "user", "content": format!("Generate a Git commit message for this diff:\n{}", diff)}
            ],
            "temperature": 0.2
        });

        let res = client
            .post("https://api.anthropic.com/v1/messages")
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        let message = json["content"][0]["text"]
            .as_str()
            .unwrap_or("Generated commit message")
            .to_string();

        Ok(message.trim().to_string())
    }
}
