//! Regex rename popup component.
//!
//! Renders a dialog for regex-based find/replace renaming.
//! Per D-03, uses state-slice signature for testability.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::state::{RegexFocus, RenameState};

use super::super::{centered_rect, render_input_field};

/// Renders the regex rename popup.
pub fn render(frame: &mut Frame, rename: &RenameState) {
    let area = centered_rect(60, 55, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Find pattern input
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Replace pattern input
            Constraint::Length(1),  // Spacer
            Constraint::Length(2),  // Error message (if any)
            Constraint::Min(8),     // Preview area
            Constraint::Length(3),  // Help text
        ])
        .split(area);

    let regex = &rename.regex;

    // Title
    let title = Paragraph::new("正则替换重命名")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Helper to check if a field is focused
    let is_focused = |focus: RegexFocus| -> bool { focus == regex.focus };

    // Input fields using helper
    render_input_field(
        frame, popup[2], "查找模式 (支持 $1, $2 捕获组)",
        &regex.find, "请输入查找模式 (正则表达式)",
        is_focused(RegexFocus::Find)
    );
    render_input_field(
        frame, popup[4], "替换模式",
        &regex.replace, "请输入替换模式 (可使用 $1, $2 引用捕获组)",
        is_focused(RegexFocus::Replace)
    );

    // Error message (if any)
    if let Some(error) = &regex.error {
        let error_para = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title("错误"));
        frame.render_widget(error_para, popup[6]);
    }

    // Preview area
    let preview_start_idx = if regex.error.is_some() { 7 } else { 6 };
    let has_preview = !regex.preview.is_empty();
    let preview_title = if has_preview {
        format!("预览 ({} 个文件将重命名)", regex.preview.len())
    } else if regex.error.is_some() {
        "预览".to_string()
    } else {
        "预览 (按 Enter 生成预览)".to_string()
    };

    let preview_lines: Vec<Line> = if has_preview {
        regex.preview.iter().take(10).map(|(old, new)| {
            Line::from(vec![
                Span::styled(old, Style::default().fg(Color::Gray)),
                Span::raw(" -> "),
                Span::styled(new, Style::default().fg(Color::Green)),
            ])
        }).collect()
    } else if regex.error.is_none() && !regex.find.is_empty() {
        vec![Line::from(Span::styled("按 Enter 生成预览", Style::default().fg(Color::DarkGray)))]
    } else {
        vec![]
    };

    let preview = Paragraph::new(preview_lines)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title(preview_title));
    frame.render_widget(preview, popup[preview_start_idx]);

    // Help text
    let help_text = if has_preview {
        "Tab 切换 | Enter 执行重命名 | Esc 取消"
    } else if regex.error.is_some() {
        "修正正则表达式 | Esc 取消"
    } else {
        "Tab 切换 | Enter 生成预览 | Esc 取消"
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[popup.len() - 1]);
}
