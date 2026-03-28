//! Login dialog component.
//!
//! Renders the login popup with username/password input fields and focus states.
//! Per D-03, uses state-slice signature receiving AuthState and AsyncState.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::components::style::{centered_rect, COLOR_CYAN, COLOR_GRAY, COLOR_GREEN, COLOR_YELLOW, COLOR_WHITE};
use crate::state::{AuthState, AsyncState, LoginFocus};

/// Renders the login dialog overlay.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `auth` - Authentication state slice with username, password, and focus
/// * `async_state` - Async state slice for loading indicator
///
/// # Layout
/// ```text
/// +------------------------+
/// | 用户名:                 |
/// +------------------------+
/// | [username input]       |
/// +------------------------+
/// | 密码:                   |
/// +------------------------+
/// | [password input]       |
/// +------------------------+
/// | Tab 切换 | Enter 登录  |
/// +------------------------+
/// ```
pub fn render(frame: &mut Frame, auth: &AuthState, async_state: &AsyncState) {
    let area = centered_rect(50, 40, frame.area());
    frame.render_widget(Clear, area);

    let is_logging_in = async_state.pending_task.is_loading();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),  // Username label
            Constraint::Length(3),  // Username input
            Constraint::Length(1),  // Password label
            Constraint::Length(3),  // Password input
            Constraint::Length(1),  // Spacer
            Constraint::Length(2),  // Help text
            Constraint::Min(1),     // Bottom spacer
        ])
        .split(area);

    // Username label - Cyan+BOLD when focused, White otherwise
    let username_label_style = if auth.login_focus == LoginFocus::Username && !is_logging_in {
        Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_WHITE)
    };
    let username_label = Paragraph::new("用户名:")
        .style(username_label_style);
    frame.render_widget(username_label, layout[0]);

    // Username input field - Green border when focused
    let username_style = if auth.login_focus == LoginFocus::Username && !is_logging_in {
        Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_CYAN)
    };
    let username_border_style = if auth.login_focus == LoginFocus::Username && !is_logging_in {
        Style::default().fg(COLOR_GREEN)
    } else {
        Style::default()
    };
    let username_input = Paragraph::new(auth.username_input.as_str())
        .style(username_style)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(username_border_style));
    frame.render_widget(username_input, layout[1]);

    // Password label - Cyan+BOLD when focused, White otherwise
    let password_label_style = if auth.login_focus == LoginFocus::Password && !is_logging_in {
        Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_WHITE)
    };
    let password_label = Paragraph::new("密码:")
        .style(password_label_style);
    frame.render_widget(password_label, layout[2]);

    // Password input field (masked with asterisks)
    let password_masked: String = auth.password_input.chars().map(|_| '*').collect();
    let password_style = if auth.login_focus == LoginFocus::Password && !is_logging_in {
        Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_CYAN)
    };
    let password_border_style = if auth.login_focus == LoginFocus::Password && !is_logging_in {
        Style::default().fg(COLOR_GREEN)
    } else {
        Style::default()
    };
    let password_input = Paragraph::new(password_masked)
        .style(password_style)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(password_border_style));
    frame.render_widget(password_input, layout[3]);

    // Help text - shows loading state or keybindings
    let help_style = if is_logging_in {
        Style::default().fg(COLOR_YELLOW)
    } else {
        Style::default().fg(COLOR_GRAY)
    };

    let help_text = if is_logging_in {
        "登录中，请稍候..."
    } else {
        "Tab 切换 | Enter 登录 | Esc 取消"
    };

    let help = Paragraph::new(help_text)
        .style(help_style)
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, layout[5]);
}
