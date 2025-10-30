use crate::error::{DevoraError, Result};
use std::fs;
use std::path::Path;

pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn copy_directory<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Err(DevoraError::FileSystemError {
            path: src.to_string_lossy().to_string(),
            message: "Source directory does not exist".to_string(),
        });
    }

    if dst.exists() {
        return Err(DevoraError::FileSystemError {
            path: dst.to_string_lossy().to_string(),
            message: "Destination already exists".to_string(),
        });
    }

    fs::create_dir_all(dst)?;

    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(src).unwrap();
        let dst_path = dst.join(relative_path);

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
        } else {
            fs::copy(src_path, &dst_path)?;
        }
    }

    Ok(())
}