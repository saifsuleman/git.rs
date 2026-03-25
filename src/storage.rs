use anyhow::{Result, bail};
use std::fs::{create_dir, create_dir_all, write};
use std::path::PathBuf;
use crate::staging::{default_staging_serialized, staging_file};

const ROOT: &str = ".git-rs";

pub fn root() -> PathBuf {
    PathBuf::from(ROOT)
}
fn objects_dir() -> PathBuf {
    root().join("objects")
}
fn refs_dir() -> PathBuf {
    root().join("refs")
}
fn head_file() -> PathBuf {
    root().join("HEAD")
}
fn is_initialised() -> bool {
    root().is_dir()
}

pub fn init_repo() -> Result<()> {
    if is_initialised() {
        bail!("repository already initialised!")
    }

    create_dir_all(objects_dir())?;
    create_dir_all(refs_dir())?;
    write(head_file(), b"main")?;
    write(refs_dir().join("main"), "")?;
    write(staging_file(), default_staging_serialized()?)?;

    Ok(())
}
