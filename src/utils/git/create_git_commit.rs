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
        .edit(message) // dialoguer uses an editor to get the message
        .expect("Failed to launch commit message editor—check that your EDITOR is configured correctly.")
        .expect("Failed to retrieve edited commit message from the temporary file.");

    // Clean up the message
    let final_message = clean_commit_message(&edited_message);

    // Execute the commit
    let status = Command::new("git")
        .args(["commit", "-m", &final_message]) // The command that Git will run
        .status() // <----------------- THIS IS THE KEY!
        .context("Failed to execute git commit")?;

    if !status.success() {
        // let stderr = String::from_utf8_lossy(&output.stderr);
        // anyhow::bail!("Git commit failed:\n{}", stderr);
        anyhow::bail!("Git commit failed. Check the output above for details.");
    }
    Ok(())
}
