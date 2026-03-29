//! Rename mode selection popup component.
//!
//! Renders a popup dialog for selecting the rename mode (Smart, Manual,
//! Unified, Regex). Per D-03, uses state-slice signature for testability.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::state::{RenameMode, RenameState};

use super::super::centered_rect;

/// Renders the rename mode selection popup.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `rename` - Rename state slice containing mode_selection
/// * `config` - Configuration (unused but kept for API consistency per D-03)
///
/// # Layout
/// ```text
/// +------------------------+
/// |   选择重命名模式        |  (Title)
/// +------------------------+
/// |                        |  (Spacer)
/// +------------------------+
/// |  > 智能重命名          |  (Mode list - 2 visible)
/// |    手动重命名          |
/// +------------------------+
/// |                        |  (Spacer)
/// +------------------------+
/// |  Preview area          |  (Preview)
/// +------------------------+
/// |  Help text             |  (Help)
/// +------------------------+
/// ```
pub fn render(frame: &mut Frame, rename: &RenameState, _config: &Config) {
    let area = centered_rect(60, 50, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(1), // Spacer
            Constraint::Length(4), // Mode selection (2 visible options)
            Constraint::Length(1), // Spacer
            Constraint::Min(5),    // Preview area
            Constraint::Length(2), // Help text
        ])
        .split(area);

    // Title
    let title = Paragraph::new("选择重命名模式")
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

    // Mode selection options - find selected index
    let modes = RenameMode::all();
    let selected_idx = modes
        .iter()
        .position(|&m| m == rename.mode_selection.selected_mode)
        .unwrap_or(0);

    // Only show 2 items at a time: selected and one neighbor
    let start_idx = if selected_idx == 0 {
        0
    } else {
        selected_idx - 1
    };
    let end_idx = std::cmp::min(start_idx + 2, modes.len());

    let mode_items: Vec<ListItem> = modes
        .iter()
        .enumerate()
        .skip(start_idx)
        .take(end_idx - start_idx)
        .map(|(_i, mode)| {
            let prefix = if *mode == rename.mode_selection.selected_mode {
                "> "
            } else {
                "  "
            };
            let style = if *mode == rename.mode_selection.selected_mode {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(
                format!("{}{}", prefix, mode.as_str()),
                style,
            )))
        })
        .collect();

    let mode_list =
        List::new(mode_items).block(Block::default().borders(Borders::ALL).title(format!(
            "模式 (↑/↓ 选择，Enter 确认) - {}/{}",
            selected_idx + 1,
            modes.len()
        )));
    frame.render_widget(mode_list, popup[2]);

    // Preview area
    let preview_title = format!("预览 ({})", rename.mode_selection.selected_mode.as_str());
    let preview_lines: Vec<Line> = rename
        .mode_selection
        .preview
        .iter()
        .map(|line| Line::from(line.as_str()))
        .collect();

    let preview = Paragraph::new(preview_lines)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title(preview_title));
    frame.render_widget(preview, popup[4]);

    // Help text
    let help = Paragraph::new("↑/↓ 选择 | Enter 确认 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[5]);
}
