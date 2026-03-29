//! Directory list component for rendering the left panel.
//!
//! This component renders the directory navigation list with:
//! - Parent directory option (when not at root)
//! - Subdirectory listing
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

use super::style::{icon, ICON_DIR, ICON_DIR_FALLBACK, ICON_PARENT, ICON_PARENT_FALLBACK};

/// Renders the directory list panel.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The rectangular area to render within
/// * `nav` - Navigation state slice (directories, selection, focus)
/// * `config` - Configuration (use_nerdfont setting)
///
/// # Behavior
/// - Shows parent ".." option when not at root path
/// - Displays selection indicator "> " for selected item
/// - Highlights selection only when focus is Directory
/// - Scrolls to keep selected item visible
pub fn render(frame: &mut Frame, area: Rect, nav: &NavigationState, config: &Config) {
    let dir_icon = icon(config.use_nerdfont, ICON_DIR, ICON_DIR_FALLBACK);
    let parent_icon = icon(config.use_nerdfont, ICON_PARENT, ICON_PARENT_FALLBACK);

    // Calculate visible height (area height minus top and bottom borders)
    let visible_height = area.height.saturating_sub(2) as usize;

    if visible_height == 0 {
        return;
    }

    // Build all items with their content
    let mut all_contents: Vec<String> = Vec::new();

    // Add parent directory option if not at root
    let has_parent = nav.current_path != "/" && !nav.current_path.is_empty();
    if has_parent {
        all_contents.push(format!("{} ..", parent_icon));
    }

    // Add all subdirectories
    for d in &nav.directories {
        all_contents.push(format!("{} {}", dir_icon, d.name));
    }

    let total_items = all_contents.len();

    if total_items == 0 {
        let list = List::new(vec![ListItem::new("")])
            .block(Block::default().borders(Borders::ALL).title("目录"));
        frame.render_widget(list, area);
        return;
    }

    // Calculate scroll offset: simple approach where selected item appears at a fixed position
    // When selected_index increases, scroll_offset increases to keep it visible
    let scroll_offset = if total_items <= visible_height {
        0
    } else {
        let max_scroll = total_items - visible_height;
        // Simple: scroll_offset = selected_index (clamped to max_scroll)
        // This makes selected item always appear at the top when scrolling
        nav.selected_index.min(max_scroll)
    };

    // Build visible items with selection highlight
    // Note: skip() first, then enumerate() so idx starts from 0
    let visible_items: Vec<ListItem> = all_contents
        .into_iter()
        .skip(scroll_offset)
        .take(visible_height)
        .enumerate()
        .map(|(idx, content)| {
            let actual_index = idx + scroll_offset;
            let is_selected =
                actual_index == nav.selected_index && matches!(nav.focus, Focus::Directory);
            let prefix = if is_selected { "> " } else { "  " };
            ListItem::new(format!("{}{}", prefix, content))
        })
        .collect();

    let list = List::new(visible_items).block(Block::default().borders(Borders::ALL).title("目录"));
    frame.render_widget(list, area);
}
