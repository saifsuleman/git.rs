use crate::patch::FilePatch;
use crate::repo::root;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub parent: Option<String>,
    pub message: String,
    pub author: String,
    pub timestamp: u64,
}

pub fn load_commit(hash: &str) -> Result<Commit> {
    todo!()
}
