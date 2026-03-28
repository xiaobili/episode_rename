use openlist_tui::app::{App, Screen};
use openlist_tui::api::types::FileItem;
use openlist_tui::update::*;

#[test]
fn test_start_manual_rename_initializes_state() {
    let mut app = App::new();

    // Add some mock files
    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::ManualRename));
    assert_eq!(app.rename.manual.index, 0);
    assert_eq!(app.rename.manual.files_to_rename, vec![0, 1, 2]);
    assert_eq!(app.rename.manual.input, "Show.S01E01.mkv");
    assert!(app.rename.manual.results.is_empty());
}

#[test]
fn test_start_manual_rename_empty_files() {
    let mut app = App::new();

    start_manual_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::ManualRename));
}

#[test]
fn test_submit_manual_rename() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    // Change the input to a new name
    app.rename.manual.input = "New.Name.S01E01.mkv".to_string();

    // Submit
    submit_manual_rename(&mut app);

    // Should have recorded the rename
    assert_eq!(app.rename.manual.results.len(), 1);
    assert_eq!(app.rename.manual.results[0], ("Show.S01E01.mkv".to_string(), "New.Name.S01E01.mkv".to_string(), true));

    // Should have moved to next file
    assert_eq!(app.rename.manual.index, 1);
    assert_eq!(app.rename.manual.input, "Show.S01E02.mkv");
}

#[test]
fn test_skip_manual_rename() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    // Skip first file
    skip_manual_rename(&mut app);

    // Should not have recorded any renames
    assert!(app.rename.manual.results.is_empty());

    // Should have moved to next file
    assert_eq!(app.rename.manual.index, 1);
    assert_eq!(app.rename.manual.input, "Show.S01E02.mkv");
}

#[test]
fn test_manual_rename_completes_all_files() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    // Submit first file with new name
    app.rename.manual.input = "New.S01E01.mkv".to_string();
    submit_manual_rename(&mut app);

    // Submit second file with new name
    app.rename.manual.input = "New.S01E02.mkv".to_string();
    submit_manual_rename(&mut app);

    // Should have finished (popup closed)
    assert!(!matches!(app.ui.screen, Screen::ManualRename));
    assert!(app.rename.manual.finished);

    // Should have recorded both renames
    assert_eq!(app.rename.manual.results.len(), 2);
}

#[test]
fn test_cancel_manual_rename() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);
    app.rename.manual.input = "New.Name.mkv".to_string();

    // Cancel
    cancel_manual_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::ManualRename));
    assert!(!app.rename.manual.finished);
    assert!(app.rename.manual.results.is_empty());
    assert!(app.rename.manual.files_to_rename.is_empty());
    assert_eq!(app.rename.manual.index, 0);
    assert!(app.rename.manual.input.is_empty());
}

#[test]
fn test_delete_last_manual_rename_char() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);
    app.rename.manual.input = "Test.mkv".to_string();

    // Delete characters
    delete_last_manual_rename_char(&mut app);
    assert_eq!(app.rename.manual.input, "Test.mk");

    delete_last_manual_rename_char(&mut app);
    assert_eq!(app.rename.manual.input, "Test.m");
}

#[test]
fn test_get_manual_rename_progress() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "S01E04.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    // At start: file 1 of 4
    assert_eq!(get_manual_rename_progress(&app), (1, 4));

    // Move to next
    next_manual_rename(&mut app);
    assert_eq!(get_manual_rename_progress(&app), (2, 4));

    // Move to next
    next_manual_rename(&mut app);
    assert_eq!(get_manual_rename_progress(&app), (3, 4));
}

#[test]
fn test_get_current_manual_rename_file() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "First.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Second.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    let current = get_current_manual_rename_file(&app);
    assert!(current.is_some());
    assert_eq!(current.unwrap().name, "First.mkv");

    // Move to next
    next_manual_rename(&mut app);
    let current = get_current_manual_rename_file(&app);
    assert!(current.is_some());
    assert_eq!(current.unwrap().name, "Second.mkv");
}

#[test]
fn test_manual_rename_same_name_not_recorded() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    // Keep the same name (don't change input)
    submit_manual_rename(&mut app);

    // Should not record rename since name didn't change
    assert!(app.rename.manual.results.is_empty());

    // Should have moved to next file
    assert_eq!(app.rename.manual.index, 1);
}

#[test]
fn test_manual_rename_empty_input_not_recorded() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_manual_rename(&mut app);

    // Clear input (empty)
    app.rename.manual.input.clear();
    submit_manual_rename(&mut app);

    // Should not record rename with empty name
    assert!(app.rename.manual.results.is_empty());
}

#[test]
fn test_take_manual_rename_results_clears() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.rename.manual.results = vec![
        ("Old.mkv".to_string(), "New.mkv".to_string(), true),
    ];

    // Take results
    let results = app.take_manual_rename_results();

    assert_eq!(results.len(), 1);
    assert!(app.rename.manual.results.is_empty());
}

#[test]
fn test_manual_rename_state_transitions() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem { name: "A.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "B.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "C.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // Initial state
    assert!(!matches!(app.ui.screen, Screen::ManualRename));
    assert_eq!(app.rename.manual.index, 0);

    // Start manual rename
    start_manual_rename(&mut app);
    assert!(matches!(app.ui.screen, Screen::ManualRename));
    assert_eq!(app.rename.manual.index, 0);

    // Skip first
    skip_manual_rename(&mut app);
    assert!(matches!(app.ui.screen, Screen::ManualRename));
    assert_eq!(app.rename.manual.index, 1);

    // Submit second with new name
    app.rename.manual.input = "New_B.mkv".to_string();
    submit_manual_rename(&mut app);
    assert!(matches!(app.ui.screen, Screen::ManualRename));
    assert_eq!(app.rename.manual.index, 2);
    assert_eq!(app.rename.manual.results.len(), 1);

    // Submit third
    app.rename.manual.input = "New_C.mkv".to_string();
    submit_manual_rename(&mut app);
    assert!(!matches!(app.ui.screen, Screen::ManualRename));
    assert!(app.rename.manual.finished);
    assert_eq!(app.rename.manual.results.len(), 2);
}
