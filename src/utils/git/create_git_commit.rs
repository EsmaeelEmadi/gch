use anyhow::{Context, Result};
use dialoguer::Editor;
use std::process::Command;

use super::{clean_commit_message, get_editor};

pub fn create_git_commit(message: &str) -> Result<()> {
    let editor = get_editor()?;

    // Present editable interface
    let edited_message = Editor::new()
        .executable(&editor)
        .extension(".commitmsg")
        .edit(message)
        .unwrap()
        .unwrap();

    // Clean up the message
    let final_message = clean_commit_message(&edited_message);

    // Execute the commit
    let output = Command::new("git")
        .args(["commit", "-m", &final_message])
        .output()
        .context("Failed to execute git commit")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git commit failed:\n{}", stderr);
    }
    Ok(())
}
