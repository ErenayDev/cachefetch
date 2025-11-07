use std::{collections::VecDeque, env, fs, path::Path};

pub fn get_size(path: &Path) -> std::io::Result<u64> {
    let expanded_path = expand_path(path)?;

    let metadata = fs::symlink_metadata(&expanded_path)?;

    if metadata.is_file() {
        Ok(metadata.len())
    } else if metadata.is_dir() {
        calculate_dir_size_iterative(&expanded_path)
    } else {
        Ok(0)
    }
}

fn expand_path(path: &Path) -> std::io::Result<std::path::PathBuf> {
    if let Some(path_str) = path.to_str() {
        if let Some(stripped) = path_str.strip_prefix('~') {
            let home = env::var("HOME")
                .or_else(|_| env::var("USERPROFILE"))
                .map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
                })?;
            let remaining = if let Some(without_slash) = stripped.strip_prefix('/') {
                without_slash
            } else {
                stripped
            };
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
    }

    Ok(path.to_path_buf())
}

fn calculate_dir_size_iterative(path: &Path) -> std::io::Result<u64> {
    let mut total = 0;
    let mut dirs_to_process = VecDeque::new();
    dirs_to_process.push_back(path.to_path_buf());

    while let Some(current_dir) = dirs_to_process.pop_front() {
        match fs::read_dir(&current_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    match fs::symlink_metadata(entry.path()) {
                        Ok(metadata) => {
                            if metadata.is_file() {
                                total += metadata.len();
                            } else if metadata.is_dir() && !metadata.file_type().is_symlink() {
                                dirs_to_process.push_back(entry.path());
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(total)
}
