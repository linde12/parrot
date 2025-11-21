use crate::structure::ParrotData;

pub fn run(tag: Option<String>, data: &ParrotData) {
    if let Some(t) = tag {
        if !recording_exists(&t, data) {
            println!("No recording found with tag: {}", t);
            return;
        }
        play_recording(&t, data);
    } else {
        todo!("Show all recordings using fzf or similar");
    }
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
