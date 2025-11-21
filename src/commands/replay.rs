use crate::structure::ParrotData;
use std::io::Write;
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("Unable to select recording with fzf")]
    SelectionError,
}

pub fn run(tag: Option<String>, data: &ParrotData) -> Result<(), ReplayError> {
    if let Some(t) = tag {
        if !recording_exists(&t, data) {
            println!("No recording found with tag: {}", t);
            return Ok(());
        }
        play_recording(&t, data);
    } else {
        // Spawn fzf and send all tags to it for selection
        let tags: Vec<&String> = data.recordings.keys().collect();
        if tags.is_empty() {
            println!("No recordings available to replay.");
            return Ok(());
        }

        let mut fzf = Command::new("fzf")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|_| ReplayError::SelectionError)?;

        let mut stdin = fzf.stdin.take().ok_or(ReplayError::SelectionError)?;
        for tag in &tags {
            writeln!(stdin, "{}", tag).expect("Failed to write to fzf stdin");
        }

        let output = fzf
            .wait_with_output()
            .map_err(|_| ReplayError::SelectionError)?;

        if output.status.success() {
            let selected_tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
            play_recording(&selected_tag, data);
        } else {
            println!("No tag selected.");
        }
    }

    Ok(())
}

fn recording_exists(tag: &str, data: &ParrotData) -> bool {
    data.recordings.contains_key(tag)
}

fn play_recording(tag: &str, data: &ParrotData) {
    if let Some(commands) = data.recordings.get(tag) {
        for command in commands {
            println!("{command}");
        }
    }
}
