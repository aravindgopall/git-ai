use super::LLMProvider;
use async_trait::async_trait;

pub struct OpenAIProvider;

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        call_openai_api(
            diff,
            "You are a Git commit message generator. Write clear, concise Git commit messages.",
        )
        .await
    }

    async fn generate_project_scaffolding(
        prompt: &str,
        input: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        call_openai_api(input, prompt).await
    }
}

async fn call_openai_api(
    input: &str,
    system_prompt: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let body = serde_json::json!({
        "model": "gpt-4",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": input}
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
        .unwrap_or("Generated message")
        .to_string();

    Ok(message.trim().to_string())
}
