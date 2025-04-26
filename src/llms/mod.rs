pub mod azure;
pub mod backend;
pub mod claude;
pub mod gemini;
pub mod ollama;
pub mod openai;

use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider {
    async fn generate_commit_message(
        diff: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn generate_project_scaffolding(
        prompt: &str,
        input: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
