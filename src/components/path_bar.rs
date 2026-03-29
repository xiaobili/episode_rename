//! Path bar component for displaying current navigation path.
//!
//! Renders the path bar showing the current directory path and navigation hints.
//! Uses state-slice signature per D-03.

use super::style::COLOR_YELLOW;
use crate::state::NavigationState;
use ratatui::{style::Style, widgets::Paragraph, Frame};

/// Renders the path bar with current navigation information.
///
/// Displays "路径：/" at root, or "路径：{path} (按 h 或 ← 返回上级)" when
/// navigating in subdirectories.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The area to render the path bar in
/// * `nav` - Navigation state slice containing path info
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, nav: &NavigationState) {
    // Build path display with parent indicator if not at root
    let path_display = if nav.current_path == "/" || nav.current_path.is_empty() {
        "/".to_string()
    } else {
        format!("{}", nav.current_path)
    };

    let path_text = if nav.current_path != "/" && !nav.current_path.is_empty() {
        format!(" 路径：{} (按 h 或 ← 返回上级)", path_display)
    } else {
        format!(" 路径：{}", path_display)
    };

    let path = Paragraph::new(path_text).style(Style::default().fg(COLOR_YELLOW));

    frame.render_widget(path, area);
}
