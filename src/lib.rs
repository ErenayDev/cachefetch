pub mod cache_folders;
pub mod get_os;
pub mod get_size;

use byte_unit::Byte;
use get_os::detect_system;
use get_size::get_size;
use std::path::Path;
use terminal_size::{Height, Width, terminal_size};

pub fn create_progress_bar(current: u64, max: u64, width: usize) -> String {
    if max == 0 {
        return " ".repeat(width);
    }
    let ratio = (current as f64 / max as f64).min(1.0);
    let total_chars = ratio * width as f64;
    let filled_chars = total_chars as usize;
    let remaining_fraction = total_chars - filled_chars as f64;
    let chars = ['█', '▉', '▊', '▋', '▌', '▍', '▎', '▏'];
    let mut bar = "█".repeat(filled_chars);
    if filled_chars < width && remaining_fraction > 0.0 {
        let char_index = ((1.0 - remaining_fraction) * chars.len() as f64) as usize;
        bar.push(chars[char_index.min(chars.len() - 1)]);
    }
    let remaining_width = width.saturating_sub(bar.chars().count());
    bar.push_str(&" ".repeat(remaining_width));
    bar
}

pub fn truncate_path(path: &str, max_width: usize) -> String {
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= max_width {
        return path.to_string();
    }
    let start = chars.len().saturating_sub(max_width.saturating_sub(3));
    format!("...{}", chars[start..].iter().collect::<String>())
}

pub fn format_size(size: u64) -> String {
    let byte_unit = Byte::from_u64(size).get_appropriate_unit(byte_unit::UnitType::Binary);
    format!("{:.2}", byte_unit)
}

pub fn run_cachefetch() -> Result<(), Box<dyn std::error::Error>> {
    let folders = detect_system();
    let mut cache_data: Vec<(String, u64)> = folders
        .iter()
        .filter_map(|folder| {
            let path = Path::new(folder);
            match get_size(path) {
                Ok(size) if size > 0 => Some((folder.to_string(), size)),
                _ => None,
            }
        })
        .collect();

    if cache_data.is_empty() {
        println!("No cache folders found.");
        return Ok(());
    }

    cache_data.sort_unstable_by_key(|b| std::cmp::Reverse(b.1));

    let terminal_width = if let Some((Width(w), Height(_))) = terminal_size() {
        w as usize
    } else {
        80
    };

    let max_size = cache_data[0].1;
    let total_size: u64 = cache_data.iter().map(|(_, size)| *size).sum();

    let bar_width = 20;
    let size_width = 12;
    let padding = 4;
    let path_width = terminal_width.saturating_sub(bar_width + size_width + padding);

    println!("Cache Files");
    println!("{}\n", "─".repeat(terminal_width));

    for (path, size) in &cache_data {
        let progress_bar = create_progress_bar(*size, max_size, bar_width);
        let truncated_path = truncate_path(path, path_width);
        let formatted_size = format_size(*size);

        println!(
            " {:<width$} {} {:>size_width$}",
            truncated_path,
            progress_bar,
            formatted_size,
            width = path_width,
            size_width = size_width
        );
    }

    println!("\n Total: {}", format_size(total_size));
    Ok(())
}
