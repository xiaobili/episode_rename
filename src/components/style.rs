//! Shared style utilities for component-based rendering.
//!
//! This module provides centralized color constants, icon helpers, and layout
//! utilities used across all UI components. Per D-08, components import from
//! this module for visual consistency.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// ============================================================================
// Color Constants
// ============================================================================

/// Cyan color for titles and highlighted elements
pub const COLOR_CYAN: ratatui::style::Color = ratatui::style::Color::Cyan;

/// Yellow color for path bars and focused input fields
pub const COLOR_YELLOW: ratatui::style::Color = ratatui::style::Color::Yellow;

/// Green color for success states and focused borders
pub const COLOR_GREEN: ratatui::style::Color = ratatui::style::Color::Green;

/// Red color for errors and warnings
pub const COLOR_RED: ratatui::style::Color = ratatui::style::Color::Red;

/// White color for normal text
pub const COLOR_WHITE: ratatui::style::Color = ratatui::style::Color::White;

/// Gray color for help text and secondary information
pub const COLOR_GRAY: ratatui::style::Color = ratatui::style::Color::Gray;

/// Dark gray color for placeholders and disabled elements
#[allow(dead_code)]
pub const COLOR_DARK_GRAY: ratatui::style::Color = ratatui::style::Color::DarkGray;

// ============================================================================
// Icon Helpers
// ============================================================================

/// Returns the appropriate icon based on Nerd Font preference.
///
/// # Arguments
/// * `use_nerdfont` - Whether Nerd Font icons are enabled
/// * `code` - The Nerd Font unicode code (e.g., "\u{f07b}")
/// * `fallback` - The fallback text for non-Nerd Font terminals
///
/// # Example
/// ```ignore
/// let dir_icon = icon(true, "\u{f07b}", "[DIR]");
/// ```
pub fn icon<'a>(use_nerdfont: bool, code: &'a str, fallback: &'a str) -> &'a str {
    if use_nerdfont {
        code
    } else {
        fallback
    }
}

/// Directory icon (folder)
pub const ICON_DIR: &str = "\u{f07b}";
/// Directory icon fallback for non-Nerd Font terminals
pub const ICON_DIR_FALLBACK: &str = "[DIR]";

/// Parent directory icon (arrow up)
pub const ICON_PARENT: &str = "\u{f062}";
/// Parent directory icon fallback
pub const ICON_PARENT_FALLBACK: &str = "[UP]";

/// Video file icon
pub const ICON_VIDEO: &str = "\u{f1c8}";
/// Video file icon fallback
pub const ICON_VIDEO_FALLBACK: &str = "[VID]";

// ============================================================================
// Layout Helpers
// ============================================================================

/// Creates a centered rectangle within the given area.
///
/// This is useful for creating popup dialogs and overlays that are centered
/// on the screen.
///
/// # Arguments
/// * `percent_x` - The width of the centered area as a percentage of the parent
/// * `percent_y` - The height of the centered area as a percentage of the parent
/// * `area` - The parent area to center within
///
/// # Returns
/// A `Rect` representing the centered area
///
/// # Example
/// ```ignore
/// use ratatui::layout::Rect;
/// let area = Rect::new(0, 0, 80, 24);
/// let popup_area = centered_rect(50, 40, area);
/// ```
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(v[1])[1]
}

// ============================================================================
// Input Field Rendering Helper
// ============================================================================

/// Renders a labeled input field with focus styling.
///
/// This helper eliminates duplicated input field rendering code across rename components.
/// Per D-17, shared rendering patterns are extracted to style module.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The area to render the input field in
/// * `label` - The title/label for the input field
/// * `value` - The current value of the input field
/// * `placeholder` - Placeholder text shown when value is empty and not focused
/// * `is_focused` - Whether this field has focus
pub fn render_input_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    placeholder: &str,
    is_focused: bool,
) {
    let text_style = if is_focused {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let border_style = if is_focused {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let display_text = if value.is_empty() && !is_focused {
        placeholder
    } else {
        value
    };
    let text_color = if value.is_empty() && !is_focused {
        Style::default().fg(Color::DarkGray)
    } else {
        text_style
    };
    let para = Paragraph::new(display_text)
        .style(text_color)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(label)
            .border_style(border_style));
    frame.render_widget(para, area);
}

/// Renders an input field with a default value shown when empty.
///
/// Similar to `render_input_field`, but shows the default value instead of
/// a placeholder when the field is empty and focused.
#[allow(dead_code)]
pub fn render_input_field_with_default(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    default: &str,
    is_focused: bool,
) {
    let text = if value.is_empty() { default } else { value };
    render_input_field(frame, area, label, text, default, is_focused);
}
