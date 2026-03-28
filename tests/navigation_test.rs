use openlist_tui::app::{App, Focus};
use openlist_tui::config::Config;
use openlist_tui::update::*;

#[test]
fn test_initial_path_state() {
    let app = App::new();
    assert_eq!(app.navigation.current_path, "/");
    assert!(app.navigation.path_history.is_empty());
}

#[test]
fn test_enter_directory_updates_path() {
    let mut app = App::new();

    // Enter a directory
    enter_directory(&mut app, "Movies");

    assert_eq!(app.navigation.current_path, "/Movies");
    assert_eq!(app.navigation.path_history.len(), 1);
    assert_eq!(app.navigation.path_history[0], "/");
}

#[test]
fn test_enter_directory_nested_path() {
    let mut app = App::new();

    // Enter multiple directories
    enter_directory(&mut app, "Movies");
    enter_directory(&mut app, "Action");

    assert_eq!(app.navigation.current_path, "/Movies/Action");
    assert_eq!(app.navigation.path_history.len(), 2);
    assert_eq!(app.navigation.path_history[0], "/");
    assert_eq!(app.navigation.path_history[1], "/Movies");
}

#[test]
fn test_go_parent_from_root() {
    let mut app = App::new();

    // At root, go_parent should do nothing
    go_parent(&mut app);

    assert_eq!(app.navigation.current_path, "/");
    assert!(app.navigation.path_history.is_empty());
}

#[test]
fn test_go_parent_one_level() {
    let mut app = App::new();

    // Enter a directory then go back
    enter_directory(&mut app, "Movies");
    go_parent(&mut app);

    assert_eq!(app.navigation.current_path, "/");
    assert_eq!(app.navigation.path_history.len(), 2);
}

#[test]
fn test_go_parent_multiple_levels() {
    let mut app = App::new();

    // Enter multiple directories
    enter_directory(&mut app, "Movies");
    enter_directory(&mut app, "Action");
    enter_directory(&mut app, "2024");

    assert_eq!(app.navigation.current_path, "/Movies/Action/2024");

    // Go to parent
    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/Movies/Action");

    // Go to parent again
    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/Movies");

    // Go to root
    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/");
}

#[test]
fn test_path_history_is_maintained() {
    let mut app = App::new();

    // Navigate through directories
    enter_directory(&mut app, "TV Shows");
    enter_directory(&mut app, "Series A");
    go_parent(&mut app);
    enter_directory(&mut app, "Series B");

    // Verify path history is maintained
    assert!(!app.navigation.path_history.is_empty());
}

#[test]
fn test_enter_directory_resets_selection() {
    let mut app = App::new();

    // Set a non-zero selection
    app.navigation.selected_index = 5;

    // Enter directory
    enter_directory(&mut app, "Test");

    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_go_parent_resets_selection() {
    let mut app = App::new();

    enter_directory(&mut app, "Test");
    app.navigation.selected_index = 5;

    // Go to parent
    go_parent(&mut app);

    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_enter_directory_sets_focus_to_directory() {
    let mut app = App::new();

    // Set focus to file
    app.navigation.focus = Focus::File;

    // Enter directory
    enter_directory(&mut app, "Test");

    assert_eq!(app.navigation.focus, Focus::Directory);
}

#[test]
fn test_go_parent_sets_focus_to_directory() {
    let mut app = App::new();

    enter_directory(&mut app, "Test");
    app.navigation.focus = Focus::File;

    // Go to parent
    go_parent(&mut app);

    assert_eq!(app.navigation.focus, Focus::Directory);
}

#[test]
fn test_go_parent_from_deep_path() {
    let mut app = App::new();

    // Create a deep path
    enter_directory(&mut app, "a");
    enter_directory(&mut app, "b");
    enter_directory(&mut app, "c");
    enter_directory(&mut app, "d");

    assert_eq!(app.navigation.current_path, "/a/b/c/d");

    // Go to parent
    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/a/b/c");

    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/a/b");

    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/a");

    go_parent(&mut app);
    assert_eq!(app.navigation.current_path, "/");
}

#[test]
fn test_with_config_navigation_state() {
    let config = Config::default();
    let app = App::with_config(config);

    assert_eq!(app.navigation.current_path, "/");
    assert!(app.navigation.path_history.is_empty());
}
