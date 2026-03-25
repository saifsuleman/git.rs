use std::collections::HashMap;
use crate::patch::FilePatch;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub parent: Option<String>,
    pub message: String,
    pub author: String,
    pub timestamp: u64,
    pub patches: FilePatch,
}