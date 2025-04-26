pub fn parse_prompt_to_ignores(prompt: &str) -> Vec<String> {
    let mut ignores = Vec::new();

    let lowered = prompt.to_lowercase();
    if lowered.contains("path") {
        ignores.push("/".to_string());
        ignores.push("\\\\".to_string());
        ignores.push(".path".to_string());
        ignores.push("C:\\".to_string());
    }
    if lowered.contains("timestamp") || lowered.contains("date") {
        ignores.push("created_at".to_string());
        ignores.push("updated_at".to_string());
        ignores.push(r"\d{4}-\d{2}-\d{2}".to_string());
    }
    if lowered.contains("localhost") {
        ignores.push("localhost".to_string());
        ignores.push("127.0.0.1".to_string());
    }

    ignores
}
