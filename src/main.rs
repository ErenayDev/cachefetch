mod cache_folders;
mod get_size;

use byte_unit::Byte;
use cache_folders::FOLDERS;
use get_size::get_size;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Row, Table},
};
use std::{io, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let cache_data: Vec<(String, String)> = FOLDERS
        .iter()
        .map(|folder| {
            let path = Path::new(folder);
            let size = get_size(path);
            let size_formatted =
                Byte::from_u64(size.unwrap_or(0)).get_appropriate_unit(byte_unit::UnitType::Binary);

            (path.display().to_string(), format!("{:.2}", size_formatted))
        })
        .collect();

    terminal.draw(|f| {
        let rows: Vec<Row> = cache_data
            .iter()
            .map(|(path, size)| Row::new(vec![path.clone(), size.clone()]))
            .collect();

        let table = Table::new(rows, [Constraint::Min(50), Constraint::Min(10)])
            .header(
                Row::new(vec!["Cache Folder", "Size"]).style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White));

        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(cache_data.len() as u16 + 2)])
            .split(f.area())[0];

        f.render_widget(table, area);
    })?;

    Ok(())
}
