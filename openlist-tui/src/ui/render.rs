use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;

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
    let path = Paragraph::new(format!(" 路径：{}", app.current_path))
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
    let items: Vec<ListItem> = app
        .directories
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let prefix = if i == app.selected_index
                && matches!(app.focus, crate::app::Focus::Directory)
            {
                "> "
            } else {
                "  "
            };
            ListItem::new(format!("{}{} {}", prefix, icon, d.name))
        })
        .collect();
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
