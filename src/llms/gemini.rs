use super::LLMProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct GeminiProvider;

#[async_trait]
impl LLMProvider for GeminiProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

        let body = json!({
            "contents": [{
                "role": "user",
                "parts": [{
                    "text": format!("Generate a concise Git commit message for this diff:\n{}", diff)
                }]
            }]
        });

        let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro:generateContent?key={}", api_key))
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        let message = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Generated commit message")
            .to_string();

        Ok(message.trim().to_string())
    }
}
