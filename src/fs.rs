use std::{fs::File, io::stdout};
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;
use error_stack::{Result, IntoReport, ResultExt, Report};


#[derive(Debug, thiserror::Error)]
pub enum FSOperationError {
    #[error("File IO error")]
    FileIO,
    #[error("Unsupported file extension: {0}")]
    FileExtension(String),
    #[error("Files without extension are not supported")]
    NullExtension,
    #[error("Bad file name")]
    BadFileName,
}


pub fn save(path: &Path, value: &impl Serialize) -> Result<(), FSOperationError> {
    match path.extension() {
        Some(x) => match x.to_str().ok_or(Report::new(FSOperationError::BadFileName))? {
            "yaml" => save_yaml(path, value),
            "json" => save_json(path, value),
            other => Err(Report::new(FSOperationError::FileExtension(other.to_string())))
        },
        None => Err(Report::new(FSOperationError::NullExtension))
    }
}


pub fn load<T: DeserializeOwned>(path: &Path) -> Result<T, FSOperationError> {
    match path.extension() {
        Some(x) => match x.to_str().ok_or(Report::new(FSOperationError::BadFileName))? {
            "yaml" => load_yaml(path),
            "json" => load_json(path),
            other => Err(Report::new(FSOperationError::FileExtension(other.to_string())))
        },
        None => Err(Report::new(FSOperationError::NullExtension))
    }
}


pub fn print_yaml(value: &impl Serialize) -> Result<(), FSOperationError> {
    let writer = stdout();

    serde_json::to_writer_pretty(writer, value)
        .into_report()
        .change_context(FSOperationError::FileIO)?;

    Ok(())
}


pub fn save_json(path: &Path, value: &impl Serialize) -> Result<(), FSOperationError> {
    let writer = create_file(path)?;
    
    serde_json::to_writer_pretty(writer, value)
        .into_report()
        .change_context(FSOperationError::FileIO)?;

    Ok(())
}


pub fn save_yaml(path: &Path, value: &impl Serialize) -> Result<(), FSOperationError> {
    let writer = create_file(path)?;
    
    serde_json::to_writer_pretty(writer, value)
        .into_report()
        .change_context(FSOperationError::FileIO)?;

    Ok(())
}


pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, FSOperationError> {
    let reader = open_file(path)?;

    serde_json::from_reader(reader)
        .into_report()
        .change_context(FSOperationError::FileIO)
}


pub fn load_yaml<T: DeserializeOwned>(path: &Path) -> Result<T, FSOperationError> {
    let reader = open_file(path)?;
    
    serde_yaml::from_reader(reader)
        .into_report()
        .change_context(FSOperationError::FileIO)
}


pub fn open_file(path: &Path) -> Result<File, FSOperationError> {
    File::open(path)
        .into_report()
        .change_context(FSOperationError::FileIO)
}


pub fn create_file(path: &Path) -> Result<File, FSOperationError> {
    File::create(path)
        .into_report()
        .change_context(FSOperationError::FileIO)
}