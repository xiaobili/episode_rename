//! Unified naming popup component.
//!
//! Renders a dialog for unified naming pattern across multiple files.
//! Per D-03, uses state-slice signature for testability.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::{RenameState, UnifiedFocus};

use super::super::{centered_rect, render_input_field};

/// Renders the unified naming popup.
pub fn render(frame: &mut Frame, rename: &RenameState) {
    let area = centered_rect(60, 60, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Show name input
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Season input
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Start episode input
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Pattern input
            Constraint::Length(1),  // Spacer
            Constraint::Min(5),     // Preview area
            Constraint::Length(2),  // Help text
        ])
        .split(area);

    let unified = &rename.unified;

    // Title
    let title = Paragraph::new("统一命名")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Helper to check if a field is focused
    let is_focused = |focus: UnifiedFocus| -> bool { focus == unified.focus };

    // Input fields using helper
    render_input_field(
        frame, popup[2], "剧集名称",
        &unified.show_name, "请输入剧集名称",
        is_focused(UnifiedFocus::ShowName)
    );
    render_input_field(
        frame, popup[4], "季数 (S01)",
        &unified.season, "1",
        is_focused(UnifiedFocus::Season)
    );
    render_input_field(
        frame, popup[6], "起始集数 (E01)",
        &unified.start_episode, "1",
        is_focused(UnifiedFocus::StartEpisode)
    );
    render_input_field(
        frame, popup[8], "命名格式 ({title}, {season}, {episode})",
        &unified.pattern, "",
        is_focused(UnifiedFocus::Pattern)
    );

    // Preview area
    let preview_lines: Vec<Line> = unified.preview.iter()
        .map(|line| Line::from(line.as_str()))
        .collect();
    let preview = Paragraph::new(preview_lines)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("预览"));
    frame.render_widget(preview, popup[10]);

    // Help text
    let help = Paragraph::new("Tab 切换 | Enter 执行 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[11]);
}
