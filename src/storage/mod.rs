use thiserror::Error;

use crate::structure::ParrotData;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO Error")]
    Io,

    #[error("Write Error")]
    Write,
}

pub trait Storage {
    fn save(&mut self, data: &ParrotData) -> Result<(), StorageError>;
    fn load(&self) -> Result<ParrotData, StorageError>;
}

pub mod json;
