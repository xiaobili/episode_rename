//! Help bar component for displaying keybinding hints.
//!
//! Renders the bottom help bar showing available keyboard shortcuts.
//! Uses state-slice signature per D-03 (no state needed - static content).

use super::style::COLOR_WHITE;
use ratatui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Renders the help bar with keybinding hints.
///
/// Displays static keybinding hints: "[N] 导航 [R] 重命名 [S] 刷新 [Q] 退出"
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The area to render the help bar in
///
/// Note: This component does not require any state slice as the content is static.
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let help = " [N] 导航 [R] 重命名 [S] 刷新 [Q] 退出 ";

    let p = Paragraph::new(help)
        .style(Style::default().fg(COLOR_WHITE))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(p, area);
}
