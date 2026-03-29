//! Component-based rendering module.
//!
//! This module provides a modular rendering architecture where each UI element
//! is rendered by a dedicated component function. The main `render` function
//! orchestrates the layout and delegates to individual component renderers.
//!
//! ## Architecture
//! - `style` module: Shared colors, icons, and layout utilities
//! - `status_bar` module: Top status bar with user info
//! - `path_bar` module: Current path display
//! - `help_bar` module: Bottom help bar with keybindings
//! - `directory_list` module: Left panel directory listing
//! - `file_list` module: Right panel file listing
//! - `login_dialog` module: Login popup overlay
//! - `loading_overlay` module: Loading indicator with progress
//! - `error_popup` module: Error message popup
//! - `rename` module: Rename popup components (mode, manual, unified, regex, single)
//! - `render` function: Entry point that creates layout and calls components

pub mod style;
pub mod status_bar;
pub mod path_bar;
pub mod help_bar;
pub mod directory_list;
pub mod file_list;
pub mod login_dialog;
pub mod loading_overlay;
pub mod error_popup;
pub mod rename;

// Re-export style utilities for convenience
pub use style::*;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use crate::app::App;
use crate::config::Config;
use crate::state::{NavigationState, Screen};

/// Main render function that orchestrates all UI components.
///
/// This function creates the main layout structure and delegates rendering
/// to individual component functions. Overlays (popups, dialogs) are rendered
/// last so they appear on top.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `app` - Mutable reference to application state
///
/// # Layout Structure
/// ```text
/// +------------------------+
/// |      Status Bar        |  (3 lines)
/// +------------------------+
/// |      Path Bar          |  (1 line)
/// +-----------+------------+
/// | Directory |   File     |  (min 10 lines)
/// |   List    |   List     |
/// +-----------+------------+
/// |      Help Bar          |  (3 lines)
/// +------------------------+
/// ```
pub fn render(frame: &mut Frame, app: &mut App) {
    // Create main layout structure
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Status bar
            Constraint::Length(1),   // Path bar
            Constraint::Min(10),     // Main content (directory + file lists)
            Constraint::Length(3),   // Help bar
        ])
        .split(frame.area());

    // Render status bar (top) with authentication state
    status_bar::render(frame, chunks[0], &app.auth);

    // Render path bar with navigation state
    path_bar::render(frame, chunks[1], &app.navigation);

    // Render main content (directory + file lists)
    render_main_content(frame, chunks[2], &app.navigation, &app.config);

    // Render help bar (bottom) - static content
    help_bar::render(frame, chunks[3]);

    // Overlay rendering - rendered last to appear on top
    match &app.ui.screen {
        Screen::LoginScreen => {
            login_dialog::render(frame, &app.auth, &app.async_state);
        }
        Screen::ErrorPopup { error, .. } => {
            error_popup::render(frame, error, &app.auth);
        }
        Screen::RenameModeSelection => {
            rename::mode::render(frame, &app.rename, &app.config);
        }
        Screen::ManualRename => {
            rename::manual::render(frame, &app.rename, &app.navigation);
        }
        Screen::UnifiedRename => {
            rename::unified::render(frame, &app.rename);
        }
        Screen::RegexRename => {
            rename::regex::render(frame, &app.rename);
        }
        Screen::SingleRename => {
            rename::single::render(frame, &app.rename);
        }
        Screen::FolderRename => {
            rename::folder::render(frame, &app.rename);
        }
        Screen::Normal => {}
    }

    // Loading overlay for async tasks (rendered on top of everything)
    if app.async_state.pending_task.is_loading() {
        // Don't show loading overlay on login screen (login has its own loading state)
        if !matches!(app.ui.screen, Screen::LoginScreen) {
            loading_overlay::render(frame, &app.async_state);
        }
    }
}

/// Renders the main content area with directory and file lists side by side.
///
/// This helper function splits the area horizontally and delegates to
/// directory_list and file_list components.
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The rectangular area to render within
/// * `nav` - Navigation state slice for directory and file lists
/// * `config` - Configuration for icon selection
///
/// # Layout
/// ```text
/// +-----------+------------+
/// | Directory |   File     |
/// |   List    |   List     |
/// |  (30%)    |   (70%)    |
/// +-----------+------------+
/// ```
fn render_main_content(frame: &mut Frame, area: Rect, nav: &NavigationState, config: &Config) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    directory_list::render(frame, chunks[0], nav, config);
    file_list::render(frame, chunks[1], nav, config);
}
