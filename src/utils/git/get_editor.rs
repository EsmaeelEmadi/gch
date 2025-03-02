use std::process::Command;

pub fn get_editor() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["config", "--get", "core.editor"])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        // Fallback to standard environment variables
        let editor = std::env::var("VISUAL")
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| {
                if cfg!(windows) {
                    "notepad".to_string()
                } else {
                    "vi".to_string()
                }
            });
        Ok(editor)
    }
}
