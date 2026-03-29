//! Manual rename popup component.
//!
//! Renders a dialog for manually renaming files one by one.
//! Per D-03, uses state-slice signature for testability.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::{ManualRenameState, NavigationState, RenameState};

use super::super::centered_rect;

/// Renders the manual rename popup.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `rename` - Rename state slice containing manual rename state
/// * `nav` - Navigation state slice for file list access
///
/// # Layout
/// ```text
/// +------------------------+
/// |   手动重命名           |  (Title)
/// +------------------------+
/// |                        |  (Spacer)
/// | 原文件名：xxx.mp4      |  (Current file)
/// +------------------------+
/// |                        |  (Spacer)
/// | 新文件名: [input]      |  (Input field)
/// +------------------------+
/// |                        |  (Spacer)
/// | 进度：1/5              |  (Progress)
/// +------------------------+
/// | Help text              |  (Help)
/// +------------------------+
/// ```
pub fn render(frame: &mut Frame, rename: &RenameState, nav: &NavigationState) {
    let area = centered_rect(60, 50, frame.area());
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
            Constraint::Length(2), // Progress
            Constraint::Length(2), // Help text
        ])
        .split(area);

    // Title
    let title = Paragraph::new("手动重命名")
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

    // Current file name - get from files_to_rename list
    let manual = &rename.manual;
    let current_file = get_current_manual_rename_file(manual, nav)
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
    let input = Paragraph::new(manual.input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("新文件名")
                .border_style(Style::default().fg(Color::Green)),
        );
    frame.render_widget(input, popup[4]);

    // Progress (file X of Y)
    let (current, total) = get_manual_rename_progress(manual);
    let progress = Paragraph::new(format!("进度：{}/{}", current, total))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(progress, popup[6]);

    // Help text
    let help = Paragraph::new("Enter 下一个 | 's' 跳过 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[7]);
}

/// Gets the current file being renamed in manual mode.
///
/// # Arguments
/// * `manual` - Manual rename state
/// * `nav` - Navigation state with file list
///
/// # Returns
/// The current file item if available, None otherwise.
fn get_current_manual_rename_file<'a>(
    manual: &ManualRenameState,
    nav: &'a NavigationState,
) -> Option<&'a crate::api::types::FileItem> {
    if manual.index < manual.files_to_rename.len() {
        let file_idx = manual.files_to_rename[manual.index];
        nav.files.get(file_idx)
    } else {
        None
    }
}

/// Gets the progress for manual rename (current file index, total files).
///
/// # Arguments
/// * `manual` - Manual rename state
///
/// # Returns
/// Tuple of (current index + 1, total files to rename)
fn get_manual_rename_progress(manual: &ManualRenameState) -> (usize, usize) {
    let total = manual.files_to_rename.len();
    if total == 0 {
        (0, 0)
    } else {
        (manual.index + 1, total)
    }
}
