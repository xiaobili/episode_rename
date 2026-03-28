//! File list component for rendering the right panel.
//!
//! This component renders the file list with:
//! - Video file icons
//! - File size display
//! - Selection indicator and scroll logic
//!
//! Per D-03, this component uses a state-slice signature accepting only
//! NavigationState and Config, not the full App.

use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::config::Config;
use crate::state::{Focus, NavigationState};

use super::style::{icon, ICON_VIDEO, ICON_VIDEO_FALLBACK};

/// Renders the file list panel.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The rectangular area to render within
/// * `nav` - Navigation state slice (files, selection, focus)
/// * `config` - Configuration (use_nerdfont setting)
///
/// # Behavior
/// - Shows "没有文件" when no files are present
/// - Displays selection indicator "> " for selected item
/// - Highlights selection only when focus is File
/// - Scrolls to keep selected item visible
/// - Shows file sizes in human-readable format (KB, MB, GB, TB)
pub fn render(frame: &mut Frame, area: Rect, nav: &NavigationState, config: &Config) {
    let file_icon = icon(config.use_nerdfont, ICON_VIDEO, ICON_VIDEO_FALLBACK);

    // Calculate visible height (area height minus top and bottom borders)
    let visible_height = area.height.saturating_sub(2) as usize;

    if visible_height == 0 {
        let list = List::new(vec![ListItem::new("")])
            .block(Block::default().borders(Borders::ALL).title("文件"));
        frame.render_widget(list, area);
        return;
    }

    let total_items = nav.files.len();

    if total_items == 0 {
        let list = List::new(vec![ListItem::new("没有文件")])
            .block(Block::default().borders(Borders::ALL).title("文件"));
        frame.render_widget(list, area);
        return;
    }

    // Calculate scroll offset: simple approach where selected item appears at a fixed position
    let scroll_offset = if total_items <= visible_height {
        0
    } else {
        let max_scroll = total_items - visible_height;
        nav.selected_index.min(max_scroll)
    };

    let items: Vec<ListItem> = nav
        .files
        .iter()
        .skip(scroll_offset)
        .take(visible_height)
        .enumerate()
        .map(|(i, f)| {
            let actual_index = i + scroll_offset;
            let prefix =
                if actual_index == nav.selected_index && matches!(nav.focus, Focus::File) {
                    "> "
                } else {
                    "  "
                };
            let size = format_size(f.size.unwrap_or(0));
            ListItem::new(format!("{}{} {} ({})", prefix, file_icon, f.name, size))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("文件"));
    frame.render_widget(list, area);
}

/// Formats a file size in bytes to a human-readable string.
///
/// Converts bytes to the appropriate unit (B, KB, MB, GB, TB).
///
/// # Arguments
/// * `size` - Size in bytes
///
/// # Returns
/// A formatted string like "1.5MB" or "2.0GB"
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
