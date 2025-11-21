use crate::structure::{ParrotData, RecordingEntry};
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Debug, thiserror::Error)]
pub enum RecordError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Cannot read temporary file: {0}")]
    TempFileError(String),

    #[error("Cannot spawn editor process: {0}")]
    EditorSpawnError(String),
}

pub fn interactive(tag: String, data: &mut ParrotData, editor: &str) -> Result<(), RecordError> {
    let mut temp_file = NamedTempFile::new().map_err(|e| {
        RecordError::TempFileError(format!("Failed to create temporary file: {}", e))
    })?;

    let _ = writeln!(
        temp_file,
        "# Enter your commands below. Save and exit to finish recording.\n"
    );

    let _ = Command::new(editor)
        .arg(temp_file.path())
        .status()
        .map_err(|e| {
            RecordError::EditorSpawnError(format!("Failed to spawn editor {}: {}", editor, e))
        })?;

    let content = std::fs::read_to_string(temp_file.path())
        .map_err(|_| RecordError::TempFileError("Failed to read temporary file".into()))?;

    let commands: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect();

    if commands.is_empty() {
        println!("No commands recorded.");
        return Ok(());
    }

    data.recordings.insert(tag.clone(), commands);
    println!("Recorded commands under tag: {}", tag);

    Ok(())
}

pub fn start(tag: String, data: &mut ParrotData) {
    if data.current_recording.is_some() {
        println!(
            "A recording is already in progress. Please stop or abort it before starting a new one.\n"
        );
        return;
    }

    data.current_recording = Some(RecordingEntry {
        tag: tag.clone(),
        data: Vec::new(),
    });
    println!("Started recording for tag: {}", tag);
}

pub fn add(command: String, data: &mut ParrotData) {
    data.current_recording.as_mut().map(|rec| {
        rec.data.push(command.clone());
    });
    println!("Recorded command {}", command);
}

pub fn stop(data: &mut ParrotData) {
    if let Some(current) = data.current_recording.take() {
        let tag = current.tag.clone();
        data.recordings.insert(tag.clone(), current.data);
        println!("Stopped recording for tag: {}", tag);
    } else {
        println!("No active recording to stop.");
    }
}

pub fn abort(data: &mut ParrotData) {
    data.current_recording = None;
    println!("Aborted current recording.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::ParrotData;
    #[test]
    fn test_recording_flow() {
        let mut data = ParrotData::default();
        start("test".to_string(), &mut data);
        add("echo Hello".to_string(), &mut data);
        add("ls -la".to_string(), &mut data);
        stop(&mut data);
        assert_eq!(
            data.recordings.get("test").unwrap(),
            &vec!["echo Hello".to_string(), "ls -la".to_string()]
        );
    }

    #[test]
    fn test_abort_recording() {
        let mut data = ParrotData::default();
        start("test".to_string(), &mut data);
        add("echo Hello".to_string(), &mut data);
        abort(&mut data);
        assert!(data.recordings.get("test").is_none());
        assert!(data.current_recording.is_none());
    }

    #[test]
    fn test_stop_without_start() {
        let mut data = ParrotData::default();
        stop(&mut data);
        assert!(data.recordings.is_empty());
    }

    #[test]
    fn test_add_without_start() {
        let mut data = ParrotData::default();
        add("echo Hello".to_string(), &mut data);
        assert!(data.recordings.is_empty());
    }

    #[test]
    fn test_start_when_already_recording() {
        let mut data = ParrotData::default();
        start("test1".to_string(), &mut data);
        start("test2".to_string(), &mut data);
        assert_eq!(data.current_recording.as_ref().unwrap().tag, "test1");
    }
}
