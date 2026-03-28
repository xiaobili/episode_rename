//! Error popup component for displaying API and application errors.
//!
//! This component renders a centered popup dialog showing error details including
//! error type, message, optional error code, and context-sensitive help text.
//!
//! ## State-Slice Signature
//! Per D-03, this component uses a state-slice signature for testability:
//! `render(frame, error, auth)` - only takes the state it needs.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::components::style::{centered_rect, COLOR_GRAY, COLOR_RED, COLOR_WHITE, COLOR_YELLOW};
use crate::state::auth::AuthState;
use crate::state::ui::ErrorInfo;

/// Renders an error popup dialog centered on the screen.
///
/// This function displays error information in a popup overlay with:
/// - Error type (red, bold)
/// - Error message (white, bordered with title "错误详情")
/// - Error code (yellow, if present)
/// - Context-sensitive help text (gray)
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `error` - Error information containing message, type, and optional code
/// * `auth` - Authentication state for checking token expiration
///
/// # Layout
/// ```text
/// +---------------------------+
/// | 错误类型：[type]          |  (red, bold)
/// +---------------------------+
/// |                           |
/// |     错误详情              |
/// |   [error message]         |  (white, bordered)
/// |                           |
/// +---------------------------+
/// | 错误代码：[code]          |  (yellow, if present)
/// +---------------------------+
/// | [Help text]               |  (gray)
/// +---------------------------+
/// ```
pub fn render(frame: &mut Frame, error: &ErrorInfo, auth: &AuthState) {
    // Create centered popup area (50% width, 40% height)
    let area = centered_rect(50, 40, frame.area());
    frame.render_widget(Clear, area);

    // Create popup layout with 6 rows
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1), // Error type
            Constraint::Length(1), // Spacer
            Constraint::Min(3),    // Error message
            Constraint::Length(1), // Spacer
            Constraint::Length(2), // Error code (if available)
            Constraint::Length(3), // Help text
        ])
        .split(area);

    // Error type (red + bold)
    let error_type = error.error_type.as_deref().unwrap_or("错误");
    let type_para = Paragraph::new(format!("错误类型：{}", error_type))
        .style(
            ratatui::style::Style::default()
                .fg(COLOR_RED)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(type_para, popup_layout[0]);

    // Error message (white text, bordered, title "错误详情")
    let msg_para = Paragraph::new(error.message.as_str())
        .style(ratatui::style::Style::default().fg(COLOR_WHITE))
        .block(Block::default().borders(Borders::ALL).title("错误详情"));
    frame.render_widget(msg_para, popup_layout[2]);

    // Error code (yellow, if present)
    if let Some(code) = error.error_code {
        let code_para = Paragraph::new(format!("错误代码：{}", code))
            .style(ratatui::style::Style::default().fg(COLOR_YELLOW));
        frame.render_widget(code_para, popup_layout[4]);
    }

    // Help text - context-sensitive based on token expiration and message content
    let help_text = if auth.is_token_expired {
        "Token 已过期，请重新登录 | Enter 前往登录 | Esc 关闭"
    } else if error.message.contains("网络") {
        "Enter 重试 | Esc 关闭"
    } else {
        "Enter 关闭"
    };

    let help = Paragraph::new(help_text)
        .style(ratatui::style::Style::default().fg(COLOR_GRAY))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup_layout[5]);
}
