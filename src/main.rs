mod cache_folders;
mod get_os;
mod get_size;

use byte_unit::Byte;
use get_os::detect_system;
use get_size::get_size;
use std::path::Path;

use terminal_size::{Height, Width, terminal_size};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folders = detect_system();
    let mut cache_data: Vec<(String, u64)> = folders
        .iter()
        .filter_map(|folder| {
            let path = Path::new(folder);
            if let Ok(size) = get_size(path) {
                if size > 0 {
                    Some((folder.to_string(), size))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    cache_data.sort_by(|a, b| b.1.cmp(&a.1));

    let terminal_width = if let Some((Width(w), Height(_))) = terminal_size() {
        w as usize
    } else {
        80
    };

    let size_column_width = 15;
    let padding = 3;
    let path_column_width = terminal_width.saturating_sub(size_column_width + padding);

    println!(
        "{:<width$} {:>15}",
        "Cache Folder",
        "Size",
        width = path_column_width
    );
    println!("{}", "─".repeat(terminal_width));

    for (path, size) in cache_data {
        let size_formatted = Byte::from_u64(size).get_appropriate_unit(byte_unit::UnitType::Binary);
        let truncated_path = if path.len() > path_column_width {
            format!(
                "...{}",
                &path[path.len().saturating_sub(path_column_width - 3)..]
            )
        } else {
            path
        };
        println!(
            "{:<width$} {:>15.2}",
            truncated_path,
            size_formatted,
            width = path_column_width
        );
    }

    Ok(())
}
