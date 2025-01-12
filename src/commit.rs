use std::process::Command;

pub fn create_commit(message: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()?;
    Ok(())
}
