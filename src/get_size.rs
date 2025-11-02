use std::{env, fs, path::Path};

pub fn get_size(path: &Path) -> std::io::Result<u64> {
    let expanded_path = if path.starts_with("~") {
        if let Ok(home) = env::var("HOME") {
            Path::new(&home).join(path.strip_prefix("~").unwrap())
        } else if let Ok(home) = env::var("USERPROFILE") {
            Path::new(&home).join(path.strip_prefix("~").unwrap())
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME/USERPROFILE environment variable not found",
            ));
        }
    } else if path.starts_with("%") && path.ends_with("%") {
        let var_name = path.to_str().unwrap().trim_matches('%');
        let var_value = env::var(var_name).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{} environment variable not found", var_name),
            )
        })?;
        Path::new(&var_value).to_path_buf()
    } else {
        path.to_path_buf()
    };

    let metadata = fs::metadata(&expanded_path)?;
    if metadata.is_file() {
        Ok(metadata.len())
    } else if metadata.is_dir() {
        calculate_dir_size(&expanded_path)
    } else {
        Ok(0)
    }
}

fn calculate_dir_size(path: &Path) -> std::io::Result<u64> {
    let mut total = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry
                && let Ok(size) = get_size(&entry.path()) {
                    total += size;
                }
        }
    }
    Ok(total)
}
