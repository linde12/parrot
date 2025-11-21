use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use crate::{
    storage::{Storage, StorageError},
    structure::ParrotData,
};

pub struct JsonFileStorage<'a> {
    file_path: &'a PathBuf,
}

impl<'a> JsonFileStorage<'a> {
    pub fn new(file_path: &'a PathBuf) -> Self {
        JsonFileStorage {
            file_path: file_path,
        }
    }
}

impl<'a> Storage for JsonFileStorage<'a> {
    fn save(&mut self, data: &ParrotData) -> Result<(), StorageError> {
        let f = File::create(&self.file_path).map_err(|_| StorageError::Io)?;
        serde_json::to_writer_pretty(f, &data).map_err(|_| StorageError::Write)?;
        Ok(())
    }

    fn load(&self) -> Result<ParrotData, StorageError> {
        if self.file_path.exists() == false {
            OpenOptions::new()
                .read(true)
                .create(true)
                .write(true)
                .open(&self.file_path)
                .map_err(|_| StorageError::Io)?;
        }
        let file = File::open(&self.file_path).map_err(|_| StorageError::Io)?;
        let data = serde_json::from_reader(file).unwrap_or_else(|_| ParrotData {
            current_recording: None,
            recordings: std::collections::HashMap::new(),
        });
        Ok(data)
    }
}
