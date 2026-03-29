//! Folder rename popup component.
//!
//! Renders a dialog for renaming a single folder.
//! Per D-03, uses state-slice signature for testability.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::RenameState;

use super::super::centered_rect;

/// Renders the folder rename popup.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `rename` - Rename state slice containing folder rename state
///
/// # Layout (per D-09 to D-13)
/// ```text
/// +----------------------------+
/// |   文件夹重命名             |  (Title - D-09)
/// +----------------------------+
/// |                            |  (Spacer)
/// | 原文件夹名：FolderName     |  (Current folder - D-11)
/// +----------------------------+
/// |                            |  (Spacer)
/// | 新文件夹名: [input]        |  (Input field - D-12)
/// +----------------------------+
/// | [validation error if any]  |  (Inline error - D-14, D-16)
/// +----------------------------+
/// | Enter 确认 | Esc 取消      |  (Help - D-13)
/// +----------------------------+
/// ```
pub fn render(frame: &mut Frame, rename: &RenameState) {
    let area = centered_rect(60, 50, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Current folder name
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Input field
            Constraint::Length(2), // Validation error (or spacer)
            Constraint::Length(2), // Help text
        ])
        .split(area);

    let folder = &rename.folder;

    // Title - D-09: "文件夹重命名"
    let title = Paragraph::new("文件夹重命名")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    frame.render_widget(title, popup[0]);

    // Current folder name - D-11: "原文件夹名：{name}"
    let current_folder = folder
        .target
        .as_ref()
        .map(|f| f.name.as_str())
        .unwrap_or("无文件夹");
    let folder_label = Paragraph::new(format!("原文件夹名：{}", current_folder))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("当前文件夹"));
    frame.render_widget(folder_label, popup[2]);

    // Input field - D-12: "新文件夹名"
    let input_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let input = Paragraph::new(folder.input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("新文件夹名")
                .border_style(Style::default().fg(Color::Green)),
        );
    frame.render_widget(input, popup[4]);

    // Validation error (inline) - D-14, D-16
    if let Some(error) = &folder.validation_error {
        let error_text = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(error_text, popup[5]);
    }

    // Help text - D-13: "Enter 确认 | Esc 取消"
    let help = Paragraph::new("Enter 确认 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[6]);
}
