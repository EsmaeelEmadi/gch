pub fn clean_commit_message(raw: &str) -> String {
    raw.lines()
        // Remove comment lines and trim whitespace
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
