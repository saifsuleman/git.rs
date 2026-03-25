use crate::repo::root;
use std::path::PathBuf;

pub fn refs_dir() -> PathBuf {
    root().join("refs")
}
