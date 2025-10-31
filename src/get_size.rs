use std::{env, fs, path::Path};

pub fn get_size(path: &Path) -> std::io::Result<u64> {
    let expanded_path = if path.starts_with("~") {
        let home = env::var("HOME").map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME environment variable not found",
            )
        })?;
        Path::new(&home).join(path.strip_prefix("~").unwrap())
    } else {
        path.to_path_buf()
    };

    let metadata = fs::metadata(&expanded_path)?;
    if metadata.is_file() {
        Ok(metadata.len())
    } else if metadata.is_dir() {
        let mut total = 0;
        if let Ok(entries) = fs::read_dir(&expanded_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(size) = get_size(&entry.path()) {
                        total += size;
                    }
                }
            }
        }
        Ok(total)
    } else {
        Ok(0)
    }
}
