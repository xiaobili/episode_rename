//! Loading overlay component.
//!
//! Renders the loading popup with spinner animation and progress bar.
//! Per D-03, uses state-slice signature receiving AsyncState only.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, Paragraph},
    Frame,
};

use crate::components::style::{centered_rect, COLOR_CYAN, COLOR_GREEN, COLOR_YELLOW};
use crate::state::AsyncState;

/// Renders the loading overlay with spinner and optional progress bar.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `async_state` - Async state slice with pending task info
///
/// # Layout
/// ```text
/// +------------------------+
/// |       加载中            |
/// |   ⠋ 处理中...          |
/// +------------------------+
/// |   [=====>   ] 50%      |
/// +------------------------+
/// ```
///
/// The progress bar is shown when the task has progress info (e.g., renaming).
/// Otherwise, animated dots "..." are shown for indeterminate progress.
pub fn render(frame: &mut Frame, async_state: &AsyncState) {
    let area = centered_rect(50, 30, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Spinner and message
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Progress bar or dots
            Constraint::Length(1),  // Spacer
        ])
        .split(area);

    // Get spinner character and message from pending task
    let spinner_char = async_state.pending_task.get_spinner_char();
    let message = async_state.pending_task.get_message().unwrap_or("处理中...");

    // Spinner and message row
    let spinner_text = format!("{} {}", spinner_char, message);
    let spinner = Paragraph::new(spinner_text)
        .style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_CYAN))
            .title("加载中"));
    frame.render_widget(spinner, popup[1]);

    // Progress bar or indeterminate dots
    if let Some((completed, total)) = async_state.pending_task.get_progress() {
        // Determinate progress - show gauge
        let percentage = if total > 0 {
            (completed as f64 / total as f64 * 100.0).min(100.0)
        } else {
            0.0
        };

        let progress_text = format!("{}% ({}/{})", percentage as usize, completed, total);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(COLOR_GREEN))
            .percent(percentage as u16)
            .label(progress_text);
        frame.render_widget(gauge, popup[3]);
    } else {
        // Indeterminate progress - show dots
        let dots = Paragraph::new("...")
            .style(Style::default().fg(COLOR_YELLOW))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_YELLOW)));
        frame.render_widget(dots, popup[3]);
    }
}
