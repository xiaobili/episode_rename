use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, LoginFocus, RenameMode};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());
    render_status_bar(frame, app, chunks[0]);
    render_path_bar(frame, app, chunks[1]);
    render_main_content(frame, app, chunks[2]);
    render_help_bar(frame, app, chunks[3]);
    if app.show_error_popup {
        render_error_popup(frame, app);
    }
    if app.show_login_screen {
        render_login_dialog(frame, app);
    }
    if app.show_rename_mode_popup {
        render_rename_mode_popup(frame, app);
    }
    if app.show_manual_rename_popup {
        render_manual_rename_popup(frame, app);
    }
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let title = if app.is_authenticated {
        format!(
            "OpenList TUI - {}",
            app.current_user.as_deref().unwrap_or("未知")
        )
    } else {
        "OpenList TUI - 未登录".into()
    };
    let status = Paragraph::new(Line::from(Span::styled(
        title,
        Style::default().fg(Color::Cyan),
    )))
    .block(Block::default().borders(Borders::ALL).title("状态"));
    frame.render_widget(status, area);
}

fn render_path_bar(frame: &mut Frame, app: &App, area: Rect) {
    // Build path display with parent indicator if not at root
    let path_display = if app.current_path == "/" || app.current_path.is_empty() {
        "/".to_string()
    } else {
        // Add parent directory indicator
        format!("{}", app.current_path)
    };

    let path_text = if app.current_path != "/" && !app.current_path.is_empty() {
        format!(" 路径：{} (按 h 或 ← 返回上级)", path_display)
    } else {
        format!(" 路径：{}", path_display)
    };

    let path = Paragraph::new(path_text)
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(path, area);
}

fn render_main_content(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);
    render_directory_list(frame, app, chunks[0]);
    render_file_list(frame, app, chunks[1]);
}

fn render_directory_list(frame: &mut Frame, app: &App, area: Rect) {
    let icon = if app.config.use_nerdfont { "\u{f07b}" } else { "[DIR]" };
    let parent_icon = if app.config.use_nerdfont { "\u{f062}" } else { "[UP]" };

    let mut items: Vec<ListItem> = Vec::new();

    // Add parent directory option if not at root
    if app.current_path != "/" && !app.current_path.is_empty() {
        let prefix = if app.selected_index == 0 && matches!(app.focus, crate::app::Focus::Directory) {
            "> "
        } else {
            "  "
        };
        items.push(ListItem::new(format!("{}{} ..", prefix, parent_icon)));
    }

    // Add subdirectories
    for (i, d) in app.directories.iter().enumerate() {
        let index_offset = if app.current_path != "/" && !app.current_path.is_empty() { 1 } else { 0 };
        let prefix = if app.selected_index == i + index_offset && matches!(app.focus, crate::app::Focus::Directory) {
            "> "
        } else {
            "  "
        };
        items.push(ListItem::new(format!("{}{} {}", prefix, icon, d.name)));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("目录"));
    frame.render_widget(list, area);
}

fn render_file_list(frame: &mut Frame, app: &App, area: Rect) {
    let icon = if app.config.use_nerdfont { "\u{f1c8}" } else { "[VID]" };
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let prefix =
                if i == app.selected_index && matches!(app.focus, crate::app::Focus::File) {
                    "> "
                } else {
                    "  "
                };
            let size = format_size(f.size.unwrap_or(0));
            ListItem::new(format!("{}{} {} ({})", prefix, icon, f.name, size))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("文件"));
    frame.render_widget(list, area);
}

fn format_size(size: u64) -> String {
    const U: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut s = size as f64;
    let mut i = 0;
    while s >= 1024.0 && i < U.len() - 1 {
        s /= 1024.0;
        i += 1;
    }
    format!("{:.1}{}", s, U[i])
}

fn render_help_bar(frame: &mut Frame, _app: &App, area: Rect) {
    let help = " [N] 导航 [R] 重命名 [S] 刷新 [Q] 退出 ";
    let p = Paragraph::new(help)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(p, area);
}

fn render_error_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 30, frame.area());
    frame.render_widget(Clear, area);
    let msg = app.error_message.as_deref().unwrap_or("未知错误");
    let p = Paragraph::new(msg)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL).title("错误"));
    frame.render_widget(p, area);
}

fn centered_rect(px: u16, py: u16, area: Rect) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(v[1])[1]
}

fn render_login_dialog(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 40, frame.area());
    frame.render_widget(Clear, area);

    let _title = if app.is_logging_in { "登录中..." } else { "登录" };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(1),
        ])
        .split(area);

    // Username label
    let username_label_style = if app.login_focus == LoginFocus::Username && !app.is_logging_in {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let username_label = Paragraph::new("用户名:")
        .style(username_label_style);
    frame.render_widget(username_label, layout[0]);

    // Username input field
    let username_style = if app.login_focus == LoginFocus::Username && !app.is_logging_in {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };
    let username_border = if app.login_focus == LoginFocus::Username && !app.is_logging_in {
        Borders::ALL
    } else {
        Borders::ALL
    };
    let username_input = Paragraph::new(app.username_input.as_str())
        .style(username_style)
        .block(Block::default().borders(username_border).style(
            if app.login_focus == LoginFocus::Username && !app.is_logging_in {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            }
        ));
    frame.render_widget(username_input, layout[1]);

    // Password label
    let password_label_style = if app.login_focus == LoginFocus::Password && !app.is_logging_in {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let password_label = Paragraph::new("密码:")
        .style(password_label_style);
    frame.render_widget(password_label, layout[2]);

    // Password input field (masked)
    let password_masked: String = app.password_input.chars().map(|_| '*').collect();
    let password_style = if app.login_focus == LoginFocus::Password && !app.is_logging_in {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };
    let password_input = Paragraph::new(password_masked)
        .style(password_style)
        .block(Block::default().borders(Borders::ALL).style(
            if app.login_focus == LoginFocus::Password && !app.is_logging_in {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            }
        ));
    frame.render_widget(password_input, layout[3]);

    // Help text
    let help_style = if app.is_logging_in {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let help_text = if app.is_logging_in {
        "登录中，请稍候..."
    } else {
        "Tab 切换 | Enter 登录 | Esc 取消"
    };

    let help = Paragraph::new(help_text)
        .style(help_style)
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, layout[5]);
}

fn render_rename_mode_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 50, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Spacer
            Constraint::Length(4),  // Mode selection (4 options)
            Constraint::Length(1),  // Spacer
            Constraint::Min(5),     // Preview area
            Constraint::Length(2),  // Help text
        ])
        .split(area);

    // Title
    let title = Paragraph::new("选择重命名模式")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Mode selection options
    let modes = RenameMode::all();
    let mode_items: Vec<ListItem> = modes.iter().enumerate().map(|(_i, mode)| {
        let prefix = if *mode == app.selected_rename_mode { "> " } else { "  " };
        let style = if *mode == app.selected_rename_mode {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(Line::from(Span::styled(
            format!("{}{}", prefix, mode.as_str()),
            style,
        )))
    }).collect();

    let mode_list = List::new(mode_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("模式"));
    frame.render_widget(mode_list, popup[2]);

    // Preview area
    let preview_title = format!("预览 ({})", app.selected_rename_mode.as_str());
    let preview_lines: Vec<Line> = app.rename_preview.iter().map(|line| {
        Line::from(line.as_str())
    }).collect();

    let preview = Paragraph::new(preview_lines)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(preview_title));
    frame.render_widget(preview, popup[4]);

    // Help text
    let help = Paragraph::new("↑/↓ 选择 | Enter 确认 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[5]);
}

fn render_manual_rename_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 50, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Current file name
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Input field
            Constraint::Length(1),  // Spacer
            Constraint::Length(2),  // Progress
            Constraint::Length(2),  // Help text
        ])
        .split(area);

    // Title
    let title = Paragraph::new("手动重命名")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Current file name
    let current_file = app.get_current_manual_rename_file()
        .map(|f| f.name.as_str())
        .unwrap_or("无文件");
    let file_label = Paragraph::new(format!("原文件名：{}", current_file))
        .style(Style::default().fg(Color::White))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("当前文件"));
    frame.render_widget(file_label, popup[2]);

    // Input field for new name
    let input_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let input = Paragraph::new(app.manual_rename_input.as_str())
        .style(input_style)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("新文件名")
            .border_style(Style::default().fg(Color::Green)));
    frame.render_widget(input, popup[4]);

    // Progress (file X of Y)
    let (current, total) = app.get_manual_rename_progress();
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
