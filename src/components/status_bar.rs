//! Status bar component for displaying authentication status.
//!
//! Renders the top status bar showing the application title and current user
//! authentication state. Uses state-slice signature per D-03.

use super::style::COLOR_CYAN;
use crate::state::AuthState;
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Renders the status bar with authentication information.
///
/// Displays "OpenList TUI - {username}" when authenticated, or
/// "OpenList TUI - 未登录" when not authenticated.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The area to render the status bar in
/// * `auth` - Authentication state slice containing user info
pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, auth: &AuthState) {
    let title = if auth.is_authenticated {
        format!(
            "OpenList TUI - {}",
            auth.current_user.as_deref().unwrap_or("未知")
        )
    } else {
        "OpenList TUI - 未登录".into()
    };

    let status = Paragraph::new(Line::from(Span::styled(
        title,
        Style::default().fg(COLOR_CYAN),
    )))
    .block(Block::default().borders(Borders::ALL).title("状态"));

    frame.render_widget(status, area);
}
