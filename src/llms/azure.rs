use super::LLMProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct AzureOpenAIProvider;

#[async_trait]
impl LLMProvider for AzureOpenAIProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let endpoint =
            std::env::var("AZURE_OPENAI_ENDPOINT").expect("AZURE_OPENAI_ENDPOINT not set");
        let api_key = std::env::var("AZURE_OPENAI_API_KEY").expect("AZURE_OPENAI_API_KEY not set");
        let api_version =
            std::env::var("AZURE_OPENAI_API_VERSION").expect("AZURE_OPENAI_API_VERSION not set");
        let deployment =
            std::env::var("AZURE_OPENAI_DEPLOYMENT").expect("AZURE_OPENAI_DEPLOYMENT not set");

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            endpoint, deployment, api_version
        );

        let body = json!({
            "messages": [
                {"role": "system", "content": "You are a Git commit message generator. Write concise commits based on diffs. Don't repeat yourself be concise and only how would normal developers write."},
                {"role": "user", "content": format!("Generate a Git commit message for this diff:\n{}", diff)}
            ],
            "temperature": 0.2
        });

        let res = client
            .post(url)
            .header("api-key", api_key)
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
