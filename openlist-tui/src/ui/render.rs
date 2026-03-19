use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Gauge},
    Frame,
};
use crate::app::{App, LoginFocus, RenameMode, UnifiedFocus, RegexFocus};

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
    if app.show_unified_input {
        render_unified_naming_popup(frame, app);
    }
    if app.show_regex_input {
        render_regex_rename_popup(frame, app);
    }
    if app.show_single_rename {
        render_single_rename_popup(frame, app);
    }
    // Render loading overlay for async tasks
    if app.pending_task.is_loading() {
        render_loading_overlay(frame, app);
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
    let area = centered_rect(50, 40, frame.area());
    frame.render_widget(Clear, area);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),  // Error type
            Constraint::Length(1),  // Spacer
            Constraint::Min(3),     // Error message
            Constraint::Length(1),  // Spacer
            Constraint::Length(2),  // Error code (if available)
            Constraint::Length(3),  // Help text
        ])
        .split(area);

    // Error type
    let error_type = app.error_type.as_deref().unwrap_or("错误");
    let type_para = Paragraph::new(format!("错误类型：{}", error_type))
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(type_para, popup_layout[0]);

    // Error message
    let msg = app.error_message.as_deref().unwrap_or("未知错误");
    let msg_para = Paragraph::new(msg)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("错误详情"));
    frame.render_widget(msg_para, popup_layout[2]);

    // Error code (if available)
    if let Some(code) = app.error_code {
        let code_para = Paragraph::new(format!("错误代码：{}", code))
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(code_para, popup_layout[4]);
    }

    // Help text with token expired option
    let help_text = if app.is_token_expired {
        "Token 已过期，请重新登录 | Enter 前往登录 | Esc 关闭"
    } else if app.error_message.as_ref().map_or(false, |m| m.contains("网络")) {
        "Enter 重试 | Esc 关闭"
    } else {
        "Enter 关闭"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup_layout[5]);
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
            Constraint::Length(4),  // Mode selection (2 visible options)
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

    // Mode selection options - find selected index
    let modes = RenameMode::all();
    let selected_idx = modes.iter().position(|&m| m == app.selected_rename_mode).unwrap_or(0);

    // Only show 2 items at a time: selected and one neighbor
    let start_idx = if selected_idx == 0 { 0 } else { selected_idx - 1 };
    let end_idx = std::cmp::min(start_idx + 2, modes.len());

    let mode_items: Vec<ListItem> = modes.iter().enumerate().skip(start_idx).take(end_idx - start_idx).map(|(_i, mode)| {
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
            .title(format!("模式 (↑/↓ 选择，Enter 确认) - {}/{}", selected_idx + 1, modes.len())));
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

fn render_unified_naming_popup(frame: &mut Frame, app: &App) {
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

    // Title
    let title = Paragraph::new("统一命名")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Helper to check if a field is focused
    let is_focused = |focus: UnifiedFocus| -> bool {
        focus == app.unified_focus
    };

    // Show name input
    let show_name_style = if is_focused(UnifiedFocus::ShowName) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let show_name_border = if is_focused(UnifiedFocus::ShowName) {
        Borders::ALL
    } else {
        Borders::ALL
    };
    let show_name_border_style = if is_focused(UnifiedFocus::ShowName) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let show_name_input = Paragraph::new(
        if app.unified_show_name.is_empty() { "请输入剧集名称" } else { &app.unified_show_name }
    )
    .style(if app.unified_show_name.is_empty() && !is_focused(UnifiedFocus::ShowName) {
        Style::default().fg(Color::DarkGray)
    } else {
        show_name_style
    })
    .block(Block::default()
        .borders(show_name_border)
        .title("剧集名称")
        .border_style(show_name_border_style));
    frame.render_widget(show_name_input, popup[2]);

    // Season input
    let season_style = if is_focused(UnifiedFocus::Season) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let season_border_style = if is_focused(UnifiedFocus::Season) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let season_input = Paragraph::new(
        if app.unified_season.is_empty() { "1" } else { &app.unified_season }
    )
    .style(if app.unified_season.is_empty() && !is_focused(UnifiedFocus::Season) {
        Style::default().fg(Color::DarkGray)
    } else {
        season_style
    })
    .block(Block::default()
        .borders(Borders::ALL)
        .title("季数 (S01)")
        .border_style(season_border_style));
    frame.render_widget(season_input, popup[4]);

    // Start episode input
    let episode_style = if is_focused(UnifiedFocus::StartEpisode) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let episode_border_style = if is_focused(UnifiedFocus::StartEpisode) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let episode_input = Paragraph::new(
        if app.unified_start_episode.is_empty() { "1" } else { &app.unified_start_episode }
    )
    .style(if app.unified_start_episode.is_empty() && !is_focused(UnifiedFocus::StartEpisode) {
        Style::default().fg(Color::DarkGray)
    } else {
        episode_style
    })
    .block(Block::default()
        .borders(Borders::ALL)
        .title("起始集数 (E01)")
        .border_style(episode_border_style));
    frame.render_widget(episode_input, popup[6]);

    // Pattern input
    let pattern_style = if is_focused(UnifiedFocus::Pattern) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let pattern_border_style = if is_focused(UnifiedFocus::Pattern) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let pattern_input = Paragraph::new(app.unified_pattern.as_str())
        .style(pattern_style)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("命名格式 ({title}, {season}, {episode})")
            .border_style(pattern_border_style));
    frame.render_widget(pattern_input, popup[8]);

    // Preview area
    let preview_title = "预览";
    let preview_lines: Vec<Line> = app.unified_preview.iter().map(|line| {
        Line::from(line.as_str())
    }).collect();

    let preview = Paragraph::new(preview_lines)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(preview_title));
    frame.render_widget(preview, popup[10]);

    // Help text
    let help = Paragraph::new("Tab 切换 | Enter 执行 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[11]);
}

fn render_regex_rename_popup(frame: &mut Frame, app: &App) {
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

    // Title
    let title = Paragraph::new("正则替换重命名")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Helper to check if a field is focused
    let is_focused = |focus: RegexFocus| -> bool {
        focus == app.regex_focus
    };

    // Find pattern input
    let find_style = if is_focused(RegexFocus::Find) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let find_border_style = if is_focused(RegexFocus::Find) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let find_input = Paragraph::new(
        if app.regex_find.is_empty() { "请输入查找模式 (正则表达式)" } else { &app.regex_find }
    )
    .style(if app.regex_find.is_empty() && !is_focused(RegexFocus::Find) {
        Style::default().fg(Color::DarkGray)
    } else {
        find_style
    })
    .block(Block::default()
        .borders(Borders::ALL)
        .title("查找模式 (支持 $1, $2 捕获组)")
        .border_style(find_border_style));
    frame.render_widget(find_input, popup[2]);

    // Replace pattern input
    let replace_style = if is_focused(RegexFocus::Replace) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let replace_border_style = if is_focused(RegexFocus::Replace) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let replace_input = Paragraph::new(
        if app.regex_replace.is_empty() { "请输入替换模式 (可使用 $1, $2 引用捕获组)" } else { &app.regex_replace }
    )
    .style(if app.regex_replace.is_empty() && !is_focused(RegexFocus::Replace) {
        Style::default().fg(Color::DarkGray)
    } else {
        replace_style
    })
    .block(Block::default()
        .borders(Borders::ALL)
        .title("替换模式")
        .border_style(replace_border_style));
    frame.render_widget(replace_input, popup[4]);

    // Error message (if any)
    if let Some(error) = &app.regex_error {
        let error_para = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title("错误"));
        frame.render_widget(error_para, popup[6]);
    }

    // Preview area
    let preview_start_idx = if app.regex_error.is_some() { 7 } else { 6 };
    let preview_title = if app.has_regex_preview() {
        format!("预览 ({} 个文件将重命名)", app.regex_preview.len())
    } else if app.regex_error.is_some() {
        "预览".to_string()
    } else {
        "预览 (按 Enter 生成预览)".to_string()
    };

    let preview_lines: Vec<Line> = if app.has_regex_preview() {
        app.regex_preview.iter().take(10).map(|(old, new)| {
            let old_span = Span::styled(old, Style::default().fg(Color::Gray));
            let arrow = Span::raw(" -> ");
            let new_span = Span::styled(new, Style::default().fg(Color::Green));
            Line::from(vec![old_span, arrow, new_span])
        }).collect()
    } else if app.regex_error.is_none() && !app.regex_find.is_empty() {
        vec![Line::from(Span::styled("按 Enter 生成预览", Style::default().fg(Color::DarkGray)))]
    } else {
        vec![]
    };

    let preview = Paragraph::new(preview_lines)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(preview_title));
    frame.render_widget(preview, popup[preview_start_idx]);

    // Help text
    let help_text = if app.has_regex_preview() {
        "Tab 切换 | Enter 执行重命名 | Esc 取消"
    } else if app.regex_error.is_some() {
        "修正正则表达式 | Esc 取消"
    } else {
        "Tab 切换 | Enter 生成预览 | Esc 取消"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[popup.len() - 1]);
}

fn render_single_rename_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 45, frame.area());
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
            Constraint::Length(2),  // Help text
        ])
        .split(area);

    // Title
    let title = Paragraph::new("单文件重命名")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, popup[0]);

    // Current file name
    let current_file = app.get_single_rename_target()
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
    let input = Paragraph::new(app.single_rename_input.as_str())
        .style(input_style)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("新文件名")
            .border_style(Style::default().fg(Color::Green)));
    frame.render_widget(input, popup[4]);

    // Help text
    let help = Paragraph::new("Enter 确认 | Esc 取消")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(help, popup[6]);
}

fn render_loading_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 30, frame.area());
    frame.render_widget(Clear, area);

    let popup = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Spinner and message
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Progress bar
            Constraint::Length(1),  // Spacer
        ])
        .split(area);

    // Get spinner character and message
    let spinner_char = app.pending_task.get_spinner_char();
    let message = app.pending_task.get_message().unwrap_or("处理中...");

    // Spinner and message
    let spinner_text = format!("{} {}", spinner_char, message);
    let spinner = Paragraph::new(spinner_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("加载中"));
    frame.render_widget(spinner, popup[1]);

    // Progress bar (if available)
    if let Some((completed, total)) = app.pending_task.get_progress() {
        let percentage = if total > 0 {
            (completed as f64 / total as f64 * 100.0).min(100.0)
        } else {
            0.0
        };

        let progress_text = format!("{}% ({}/{})", percentage as usize, completed, total);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Green))
            .percent(percentage as u16)
            .label(progress_text);
        frame.render_widget(gauge, popup[3]);
    } else {
        // Indeterminate progress - show animated dots
        let dots = Paragraph::new("...")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)));
        frame.render_widget(dots, popup[3]);
    }
}
