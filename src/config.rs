use std::fs;

pub fn load_profile(profile_name: String) -> Vec<String> {
    let config_content = fs::read_to_string(".git-ai-config").unwrap_or_default();

    let mut ignores = Vec::new();
    let lines: Vec<&str> = config_content.lines().collect();

    let mut in_profile = false;
    for line in lines {
        if line.trim() == format!("[{}]", profile_name) {
            in_profile = true;
            continue;
        }
        if in_profile {
            if line.starts_with('[') {
                break;
            }
            ignores.push(line.trim().to_string());
        }
    }

    ignores
}
