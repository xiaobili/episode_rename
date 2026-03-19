use openlist_tui::app::App;
use openlist_tui::api::types::FileItem;

#[test]
fn test_start_manual_rename_initializes_state() {
    let mut app = App::new();

    // Add some mock files
    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    assert!(app.show_manual_rename_popup);
    assert_eq!(app.manual_rename_index, 0);
    assert_eq!(app.files_to_rename, vec![0, 1, 2]);
    assert_eq!(app.manual_rename_input, "Show.S01E01.mkv");
    assert!(app.manual_rename_results.is_empty());
}

#[test]
fn test_start_manual_rename_empty_files() {
    let mut app = App::new();

    app.start_manual_rename();

    assert!(!app.show_manual_rename_popup);
}

#[test]
fn test_submit_manual_rename() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    // Change the input to a new name
    app.manual_rename_input = "New.Name.S01E01.mkv".to_string();

    // Submit
    app.submit_manual_rename();

    // Should have recorded the rename
    assert_eq!(app.manual_rename_results.len(), 1);
    assert_eq!(app.manual_rename_results[0], ("Show.S01E01.mkv".to_string(), "New.Name.S01E01.mkv".to_string(), true));

    // Should have moved to next file
    assert_eq!(app.manual_rename_index, 1);
    assert_eq!(app.manual_rename_input, "Show.S01E02.mkv");
}

#[test]
fn test_skip_manual_rename() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    // Skip first file
    app.skip_manual_rename();

    // Should not have recorded any renames
    assert!(app.manual_rename_results.is_empty());

    // Should have moved to next file
    assert_eq!(app.manual_rename_index, 1);
    assert_eq!(app.manual_rename_input, "Show.S01E02.mkv");
}

#[test]
fn test_manual_rename_completes_all_files() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    // Submit first file with new name
    app.manual_rename_input = "New.S01E01.mkv".to_string();
    app.submit_manual_rename();

    // Submit second file with new name
    app.manual_rename_input = "New.S01E02.mkv".to_string();
    app.submit_manual_rename();

    // Should have finished (popup closed)
    assert!(!app.show_manual_rename_popup);
    assert!(app.manual_rename_finished);

    // Should have recorded both renames
    assert_eq!(app.manual_rename_results.len(), 2);
}

#[test]
fn test_cancel_manual_rename() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();
    app.manual_rename_input = "New.Name.mkv".to_string();

    // Cancel
    app.cancel_manual_rename();

    assert!(!app.show_manual_rename_popup);
    assert!(!app.manual_rename_finished);
    assert!(app.manual_rename_results.is_empty());
    assert!(app.files_to_rename.is_empty());
    assert_eq!(app.manual_rename_index, 0);
    assert!(app.manual_rename_input.is_empty());
}

#[test]
fn test_delete_last_manual_rename_char() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();
    app.manual_rename_input = "Test.mkv".to_string();

    // Delete characters
    app.delete_last_manual_rename_char();
    assert_eq!(app.manual_rename_input, "Test.mk");

    app.delete_last_manual_rename_char();
    assert_eq!(app.manual_rename_input, "Test.m");
}

#[test]
fn test_get_manual_rename_progress() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "S01E04.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    // At start: file 1 of 4
    assert_eq!(app.get_manual_rename_progress(), (1, 4));

    // Move to next
    app.next_manual_rename();
    assert_eq!(app.get_manual_rename_progress(), (2, 4));

    // Move to next
    app.next_manual_rename();
    assert_eq!(app.get_manual_rename_progress(), (3, 4));
}

#[test]
fn test_get_current_manual_rename_file() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "First.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Second.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    let current = app.get_current_manual_rename_file();
    assert!(current.is_some());
    assert_eq!(current.unwrap().name, "First.mkv");

    // Move to next
    app.next_manual_rename();
    let current = app.get_current_manual_rename_file();
    assert!(current.is_some());
    assert_eq!(current.unwrap().name, "Second.mkv");
}

#[test]
fn test_manual_rename_same_name_not_recorded() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    // Keep the same name (don't change input)
    app.submit_manual_rename();

    // Should not record rename since name didn't change
    assert!(app.manual_rename_results.is_empty());

    // Should have moved to next file
    assert_eq!(app.manual_rename_index, 1);
}

#[test]
fn test_manual_rename_empty_input_not_recorded() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_manual_rename();

    // Clear input (empty)
    app.manual_rename_input.clear();
    app.submit_manual_rename();

    // Should not record rename with empty name
    assert!(app.manual_rename_results.is_empty());
}

#[test]
fn test_take_manual_rename_results_clears() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.manual_rename_results = vec![
        ("Old.mkv".to_string(), "New.mkv".to_string(), true),
    ];

    // Take results
    let results = app.take_manual_rename_results();

    assert_eq!(results.len(), 1);
    assert!(app.manual_rename_results.is_empty());
}

#[test]
fn test_manual_rename_state_transitions() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "A.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "B.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "C.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // Initial state
    assert!(!app.show_manual_rename_popup);
    assert_eq!(app.manual_rename_index, 0);

    // Start manual rename
    app.start_manual_rename();
    assert!(app.show_manual_rename_popup);
    assert_eq!(app.manual_rename_index, 0);

    // Skip first
    app.skip_manual_rename();
    assert!(app.show_manual_rename_popup);
    assert_eq!(app.manual_rename_index, 1);

    // Submit second with new name
    app.manual_rename_input = "New_B.mkv".to_string();
    app.submit_manual_rename();
    assert!(app.show_manual_rename_popup);
    assert_eq!(app.manual_rename_index, 2);
    assert_eq!(app.manual_rename_results.len(), 1);

    // Submit third
    app.manual_rename_input = "New_C.mkv".to_string();
    app.submit_manual_rename();
    assert!(!app.show_manual_rename_popup);
    assert!(app.manual_rename_finished);
    assert_eq!(app.manual_rename_results.len(), 2);
}
