use crate::patch::FilePatch;
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

pub fn staging_file() -> PathBuf {
    crate::repo::root().join("staging")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StagedChange {
    Upsert(String),
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StagingArea {
    pub files: HashMap<String, StagedChange>,
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

pub fn stage_file_upsert(path: &str) -> Result<()> {
    let real = Path::new(path);

    if !real.exists() {
        bail!("file {} does not exist", path);
    }

    let content = read_to_string(real)?;
    let mut staging = load_staging()?;
    staging
        .files
        .insert(path.to_string(), StagedChange::Upsert(content));
    save_staging(&staging)?;

    Ok(())
}
