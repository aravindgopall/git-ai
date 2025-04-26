pub fn apply_ignores(diff: String, ignore_patterns: Vec<String>) -> String {
    diff.lines()
        .filter(|line| !ignore_patterns.iter().any(|pat| line.contains(pat)))
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
