use std::path::PathBuf;

use sha1::{Digest, Sha1};

use crate::repo::root;

#[derive(Debug, Clone)]
pub struct HashedObject {
    hash: String,
    contents: String,
    path: PathBuf,
}

pub fn objects_dir() -> PathBuf {
    root().join("objects")
}

pub fn create_object(contents: String) -> HashedObject {
    let mut hasher = Sha1::new();

    hasher.update(contents.clone());

    let hash = hasher.finalize();
    let hash: String = format!("{:x}", hash);
    let (dir, file) = hash.split_at(2);
    let path = objects_dir().join(dir).join(file);

    HashedObject {
        hash,
        path,
        contents,
    }
}

pub fn get_object(hash: String) -> Option<HashedObject> {
    if hash.len() < 2 {
        return None;
    }

    let (dir, file) = hash.split_at(2);
    let path = objects_dir().join(dir).join(file);

    let contents = std::fs::read_to_string(&path).ok()?;
    let result = HashedObject {
        hash,
        contents,
        path,
    };

    Some(result)
}
