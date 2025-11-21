use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Tag = String;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ParrotData {
    pub current_recording: Option<RecordingEntry>,
    pub recordings: HashMap<Tag, Vec<String>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RecordingEntry {
    pub tag: Tag,
    pub data: Vec<String>,
}
