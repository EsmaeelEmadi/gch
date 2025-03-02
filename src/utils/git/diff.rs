use std::io;
use std::process::Command;

pub fn diff() -> io::Result<String> {
    let output = Command::new("git").args(["diff", "--cached"]).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, err_msg.to_string()))
    }
}
