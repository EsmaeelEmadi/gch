use anyhow::Result;
use std::process::Command;

pub fn run(cmd: &str) -> Result<bool> {
    let (shell, flag) = if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    let status = Command::new(shell).arg(flag).arg(cmd).status()?;

    if !status.success() {
        return Ok(false);
    }

    Ok(true)
}
