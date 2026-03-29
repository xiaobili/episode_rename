//! Single file rename popup component.
//!
//! Renders a dialog for renaming a single file.
//! Per D-03, uses state-slice signature for testability.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::RenameState;

use super::super::centered_rect;

/// Renders the single file rename popup.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `rename` - Rename state slice containing single rename state
///
/// # Layout
/// ```text
/// +------------------------+
/// |   单文件重命名         |  (Title)
/// +------------------------+
/// |                        |  (Spacer)
/// | 原文件名：xxx.mp4      |  (Current file)
/// +------------------------+
/// |                        |  (Spacer)
/// | 新文件名: [input]      |  (Input field)
/// +------------------------+
/// |                        |  (Spacer)
/// | Help text              |  (Help)
/// +------------------------+
/// ```
pub fn render(frame: &mut Frame, rename: &RenameState) {
    let area = centered_rect(60, 45, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Current file name
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Input field
            Constraint::Length(1), // Spacer
            Constraint::Length(2), // Help text
        ])
        .split(area);

    let single = &rename.single;

    // Title
    let title = Paragraph::new("单文件重命名")
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

    // Current file name
    let current_file = single
        .target
        .as_ref()
        .map(|f| f.name.as_str())
        .unwrap_or("无文件");
    let file_label = Paragraph::new(format!("原文件名：{}", current_file))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("当前文件"));
    frame.render_widget(file_label, popup[2]);

    // Input field for new name
    let input_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let input = Paragraph::new(single.input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("新文件名")
                .border_style(Style::default().fg(Color::Green)),
        );
    frame.render_widget(input, popup[4]);

    // Help text
    let help = Paragraph::new("Enter 确认 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[6]);
}
