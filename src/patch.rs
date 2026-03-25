use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilePatch {
    Add {
        path: String,
        contents: String,
    },
    Modify {
        path: String,
        diff: String,
    },
    Delete { path: String },
}