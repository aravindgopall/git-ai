use super::LLMProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OpenAIProvider;

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let body = json!({
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": "You are a git commit message generator. Write clear, concise Git commit messages based on provided diffs."},
                {"role": "user", "content": format!("Generate a Git commit message for this diff:\n{}", diff)}
            ],
            "temperature": 0.2
        });

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        let message = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Generated commit message")
            .to_string();

        Ok(message.trim().to_string())
    }
}
