use openlist_tui::app::App;
use openlist_tui::config::Config;

#[test]
fn test_initial_path_state() {
    let app = App::new();
    assert_eq!(app.current_path, "/");
    assert!(app.path_history.is_empty());
}

#[test]
fn test_enter_directory_updates_path() {
    let mut app = App::new();

    // Enter a directory
    app.enter_directory("Movies");

    assert_eq!(app.current_path, "/Movies");
    assert_eq!(app.path_history.len(), 1);
    assert_eq!(app.path_history[0], "/");
}

#[test]
fn test_enter_directory_nested_path() {
    let mut app = App::new();

    // Enter multiple directories
    app.enter_directory("Movies");
    app.enter_directory("Action");

    assert_eq!(app.current_path, "/Movies/Action");
    assert_eq!(app.path_history.len(), 2);
    assert_eq!(app.path_history[0], "/");
    assert_eq!(app.path_history[1], "/Movies");
}

#[test]
fn test_go_parent_from_root() {
    let mut app = App::new();

    // At root, go_parent should do nothing
    app.go_parent();

    assert_eq!(app.current_path, "/");
    assert!(app.path_history.is_empty());
}

#[test]
fn test_go_parent_one_level() {
    let mut app = App::new();

    // Enter a directory then go back
    app.enter_directory("Movies");
    app.go_parent();

    assert_eq!(app.current_path, "/");
    assert_eq!(app.path_history.len(), 2);
}

#[test]
fn test_go_parent_multiple_levels() {
    let mut app = App::new();

    // Enter multiple directories
    app.enter_directory("Movies");
    app.enter_directory("Action");
    app.enter_directory("2024");

    assert_eq!(app.current_path, "/Movies/Action/2024");

    // Go to parent
    app.go_parent();
    assert_eq!(app.current_path, "/Movies/Action");

    // Go to parent again
    app.go_parent();
    assert_eq!(app.current_path, "/Movies");

    // Go to root
    app.go_parent();
    assert_eq!(app.current_path, "/");
}

#[test]
fn test_path_history_is_maintained() {
    let mut app = App::new();

    // Navigate through directories
    app.enter_directory("TV Shows");
    app.enter_directory("Series A");
    app.go_parent();
    app.enter_directory("Series B");

    // Verify path history is maintained
    assert!(!app.path_history.is_empty());
}

#[test]
fn test_enter_directory_resets_selection() {
    let mut app = App::new();

    // Set a non-zero selection
    app.selected_index = 5;

    // Enter directory
    app.enter_directory("Test");

    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_go_parent_resets_selection() {
    let mut app = App::new();

    app.enter_directory("Test");
    app.selected_index = 5;

    // Go to parent
    app.go_parent();

    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_enter_directory_sets_focus_to_directory() {
    use openlist_tui::app::Focus;

    let mut app = App::new();

    // Set focus to file
    app.focus = Focus::File;

    // Enter directory
    app.enter_directory("Test");

    assert_eq!(app.focus, Focus::Directory);
}

#[test]
fn test_go_parent_sets_focus_to_directory() {
    use openlist_tui::app::Focus;

    let mut app = App::new();

    app.enter_directory("Test");
    app.focus = Focus::File;

    // Go to parent
    app.go_parent();

    assert_eq!(app.focus, Focus::Directory);
}

#[test]
fn test_go_parent_from_deep_path() {
    let mut app = App::new();

    // Create a deep path
    app.enter_directory("a");
    app.enter_directory("b");
    app.enter_directory("c");
    app.enter_directory("d");

    assert_eq!(app.current_path, "/a/b/c/d");

    // Go to parent
    app.go_parent();
    assert_eq!(app.current_path, "/a/b/c");

    app.go_parent();
    assert_eq!(app.current_path, "/a/b");

    app.go_parent();
    assert_eq!(app.current_path, "/a");

    app.go_parent();
    assert_eq!(app.current_path, "/");
}

#[test]
fn test_with_config_navigation_state() {
    let config = Config::default();
    let app = App::with_config(config);

    assert_eq!(app.current_path, "/");
    assert!(app.path_history.is_empty());
}
