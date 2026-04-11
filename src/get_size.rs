use jwalk::WalkDir;
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_size(path: &Path) -> std::io::Result<u64> {
    let expanded = expand_path(path)?;
    let metadata = std::fs::symlink_metadata(&expanded)?;

    if metadata.is_file() {
        return Ok(metadata.len());
    }

    if !metadata.is_dir() {
        return Ok(0);
    }

    let total = WalkDir::new(&expanded)
        .skip_hidden(false)
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            if entry.file_type().is_file() {
                entry.metadata().ok().map(|m| m.len())
            } else {
                None
            }
        })
        .sum();

    Ok(total)
}

fn expand_path(path: &Path) -> std::io::Result<PathBuf> {
    let Some(path_str) = path.to_str() else {
        return Ok(path.to_path_buf());
    };

    if let Some(stripped) = path_str.strip_prefix('~') {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
            })?;
        let remaining = stripped
            .strip_prefix('/')
            .or_else(|| stripped.strip_prefix('\\'))
            .unwrap_or(stripped);
        return Ok(Path::new(&home).join(remaining));
    }

    if path_str.starts_with('%') && path_str.ends_with('%') && path_str.len() > 2 {
        let var_name = &path_str[1..path_str.len() - 1];
        let var_value = env::var(var_name).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Environment variable {} not found", var_name),
            )
        })?;
        return Ok(Path::new(&var_value).to_path_buf());
    }

    Ok(path.to_path_buf())
}
