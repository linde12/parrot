use crate::structure::ParrotData;

pub fn start(tag: String, data: &mut ParrotData) {
    if data.current_recording.is_some() {
        println!(
            "A recording is already in progress. Please stop or abort it before starting a new one."
        );
        return;
    }

    data.current_recording = Some(crate::structure::RecordingEntry {
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
        let entry = data
            .recordings
            .entry(current.tag.clone())
            .or_insert_with(Vec::new);
        *entry = current.data;
        println!("Stopped recording for tag: {}", current.tag);
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
