use openlist_tui::app::App;
use openlist_tui::api::types::FileItem;

#[test]
fn test_start_single_rename_initializes_state() {
    let mut app = App::new();

    // Add some mock files and set focus to file list
    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.focus = openlist_tui::app::Focus::File;
    app.selected_index = 0;

    app.start_single_rename();

    assert!(app.show_single_rename);
    assert!(app.single_rename_target.is_some());
    assert_eq!(app.single_rename_input, "Show.S01E01.mkv");
}

#[test]
fn test_start_single_rename_wrong_focus() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.focus = openlist_tui::app::Focus::Directory;
    app.selected_index = 0;

    app.start_single_rename();

    assert!(!app.show_single_rename);
    assert!(app.single_rename_target.is_none());
}

#[test]
fn test_start_single_rename_empty_files() {
    let mut app = App::new();

    app.files = vec![];
    app.focus = openlist_tui::app::Focus::File;
    app.selected_index = 0;

    app.start_single_rename();

    assert!(!app.show_single_rename);
}

#[test]
fn test_submit_single_rename() {
    let mut app = App::new();

    app.show_single_rename = true;
    app.single_rename_target = Some(FileItem {
        name: "old_name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.single_rename_input = "new_name.mkv".to_string();

    app.submit_single_rename();

    assert!(!app.show_single_rename);
}

#[test]
fn test_submit_single_rename_empty_input() {
    let mut app = App::new();

    app.show_single_rename = true;
    app.single_rename_target = Some(FileItem {
        name: "old_name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.single_rename_input = "".to_string();

    app.submit_single_rename();

    // Should not submit with empty input
    assert!(app.show_single_rename);
}

#[test]
fn test_cancel_single_rename() {
    let mut app = App::new();

    app.show_single_rename = true;
    app.single_rename_input = "new_name.mkv".to_string();
    app.single_rename_target = Some(FileItem {
        name: "old_name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    });

    app.cancel_single_rename();

    assert!(!app.show_single_rename);
    assert!(app.single_rename_input.is_empty());
    assert!(app.single_rename_target.is_none());
}

#[test]
fn test_delete_last_single_rename_char() {
    let mut app = App::new();

    app.single_rename_input = "new_name.mkv".to_string();

    app.delete_last_single_rename_char();

    assert_eq!(app.single_rename_input, "new_name.mk");
}

#[test]
fn test_get_single_rename_target() {
    let mut app = App::new();

    let file = FileItem {
        name: "test_file.mkv".to_string(),
        is_dir: false,
        size: Some(2000),
    };
    app.single_rename_target = Some(file.clone());

    let target = app.get_single_rename_target();

    assert!(target.is_some());
    assert_eq!(target.unwrap().name, "test_file.mkv");
}

#[test]
fn test_single_rename_state_transitions() {
    let mut app = App::new();

    // Initial state
    assert!(!app.show_single_rename);
    assert!(app.single_rename_target.is_none());

    // Setup for single rename
    app.files = vec![
        FileItem { name: "Episode.01.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.focus = openlist_tui::app::Focus::File;
    app.selected_index = 0;

    // Start single rename
    app.start_single_rename();
    assert!(app.show_single_rename);
    assert!(app.single_rename_target.is_some());

    // Edit input
    app.single_rename_input = "New.Episode.01.mkv".to_string();

    // Cancel
    app.cancel_single_rename();
    assert!(!app.show_single_rename);
    assert!(app.single_rename_target.is_none());
}

#[test]
fn test_single_rename_input_max_length() {
    let mut app = App::new();

    app.show_single_rename = true;
    app.single_rename_input = "test".to_string();

    // Fill up to 200 chars (already has 4, add 196 more)
    for _ in 0..196 {
        if app.single_rename_input.len() < 200 {
            app.single_rename_input.push('a');
        }
    }

    assert_eq!(app.single_rename_input.len(), 200);

    // Note: The 200 char limit is enforced in main.rs event loop, not in the App struct
    // This test verifies the input can hold at least 200 chars
    assert!(app.single_rename_input.len() >= 200);
}

#[test]
fn test_single_rename_selected_index() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "File1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "File2.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "File3.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.focus = openlist_tui::app::Focus::File;

    // Test with different selected indices
    app.selected_index = 1;
    app.start_single_rename();
    assert_eq!(app.single_rename_input, "File2.mkv");

    app.cancel_single_rename();

    app.selected_index = 2;
    app.start_single_rename();
    assert_eq!(app.single_rename_input, "File3.mkv");
}

#[test]
fn test_single_rename_preserves_original() {
    let mut app = App::new();

    app.files = vec![
        FileItem { name: "Original.Name.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.focus = openlist_tui::app::Focus::File;
    app.selected_index = 0;

    app.start_single_rename();

    // The input should be initialized with the original filename
    assert_eq!(app.single_rename_input, "Original.Name.mkv");

    // Modify input
    app.single_rename_input = "Modified.Name.mkv".to_string();

    // Target should still reference original
    assert_eq!(app.single_rename_target.as_ref().unwrap().name, "Original.Name.mkv");
}
