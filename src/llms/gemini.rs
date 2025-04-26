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
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let body = json!({
        "contents": [{
            "role": "user",
            "parts": [{
                "text": format!("{}\n{}",system_prompt,input)
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
