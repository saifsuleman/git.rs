use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilePatch {
    Created { path: String, contents: String },
    Modified { path: String, diff: String },
    Deleted { path: String },
}
