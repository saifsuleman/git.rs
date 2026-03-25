use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

pub fn staging_file() -> PathBuf {
    crate::storage::root().join("staging")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StagingArea {
    pub files: HashMap<String, String>, // file name -> contents of file at time of stage
}

pub fn default_staging_serialized() -> Result<String> {
    let staging_area = &StagingArea::default();
    let serialized = serde_json::to_string(&staging_area)?;
    Ok(serialized)
}

pub fn load_staging() -> Result<StagingArea> {
    let data = read_to_string(staging_file())?;
    let deserialized = serde_json::from_str(&data)?;
    Ok(deserialized)
}

pub fn save_staging(staging: &StagingArea) -> Result<()> {
    write(staging_file(), serde_json::to_string(staging)?)?;
    Ok(())
}

pub fn stage_file(path: &str) -> Result<()> {
    let real = Path::new(path);

    if !real.exists() {
        bail!("file: {} does not exist", real.display());
    }

    let content = read_to_string(real)?;
    let mut staging = load_staging()?;
    staging.files.insert(path.to_string(), content);
    save_staging(&staging)?;

    Ok(())
}