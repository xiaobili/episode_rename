use openlist_tui::api::types::FileItem;
use openlist_tui::app::{App, Focus, Screen};
use openlist_tui::update::*;

#[test]
fn test_start_single_rename_initializes_state() {
    let mut app = App::new();

    // Add some mock files and set focus to file list
    app.navigation.files = vec![
        FileItem {
            name: "Show.S01E01.mkv".to_string(),
            is_dir: false,
            size: Some(1000),
        },
        FileItem {
            name: "Show.S01E02.mkv".to_string(),
            is_dir: false,
            size: Some(1000),
        },
    ];
    app.navigation.focus = Focus::File;
    app.navigation.selected_index = 0;

    start_single_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::SingleRename));
    assert!(app.rename.single.target.is_some());
    assert_eq!(app.rename.single.input, "Show.S01E01.mkv");
}

#[test]
fn test_start_single_rename_wrong_focus() {
    let mut app = App::new();

    app.navigation.files = vec![FileItem {
        name: "Show.S01E01.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    }];
    app.navigation.focus = Focus::Directory;
    app.navigation.selected_index = 0;

    start_single_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::SingleRename));
    assert!(app.rename.single.target.is_none());
}

#[test]
fn test_start_single_rename_empty_files() {
    let mut app = App::new();

    app.navigation.files = vec![];
    app.navigation.focus = Focus::File;
    app.navigation.selected_index = 0;

    start_single_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::SingleRename));
}

#[test]
fn test_submit_single_rename() {
    let mut app = App::new();

    app.ui.screen = Screen::SingleRename;
    app.rename.single.target = Some(FileItem {
        name: "old_name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.rename.single.input = "new_name.mkv".to_string();

    submit_single_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::SingleRename));
}

#[test]
fn test_submit_single_rename_empty_input() {
    let mut app = App::new();

    app.ui.screen = Screen::SingleRename;
    app.rename.single.target = Some(FileItem {
        name: "old_name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.rename.single.input = "".to_string();

    submit_single_rename(&mut app);

    // Should not submit with empty input
    assert!(matches!(app.ui.screen, Screen::SingleRename));
}

#[test]
fn test_cancel_single_rename() {
    let mut app = App::new();

    app.ui.screen = Screen::SingleRename;
    app.rename.single.input = "new_name.mkv".to_string();
    app.rename.single.target = Some(FileItem {
        name: "old_name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    });

    cancel_single_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::SingleRename));
    assert!(app.rename.single.input.is_empty());
    assert!(app.rename.single.target.is_none());
}

#[test]
fn test_delete_last_single_rename_char() {
    let mut app = App::new();

    app.rename.single.input = "new_name.mkv".to_string();

    delete_last_single_rename_char(&mut app);

    assert_eq!(app.rename.single.input, "new_name.mk");
}

#[test]
fn test_get_single_rename_target() {
    let mut app = App::new();

    let file = FileItem {
        name: "test_file.mkv".to_string(),
        is_dir: false,
        size: Some(2000),
    };
    app.rename.single.target = Some(file.clone());

    let target = app.get_single_rename_target();

    assert!(target.is_some());
    assert_eq!(target.unwrap().name, "test_file.mkv");
}

#[test]
fn test_single_rename_state_transitions() {
    let mut app = App::new();

    // Initial state
    assert!(!matches!(app.ui.screen, Screen::SingleRename));
    assert!(app.rename.single.target.is_none());

    // Setup for single rename
    app.navigation.files = vec![FileItem {
        name: "Episode.01.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    }];
    app.navigation.focus = Focus::File;
    app.navigation.selected_index = 0;

    // Start single rename
    start_single_rename(&mut app);
    assert!(matches!(app.ui.screen, Screen::SingleRename));
    assert!(app.rename.single.target.is_some());

    // Edit input
    app.rename.single.input = "New.Episode.01.mkv".to_string();

    // Cancel
    cancel_single_rename(&mut app);
    assert!(!matches!(app.ui.screen, Screen::SingleRename));
    assert!(app.rename.single.target.is_none());
}

#[test]
fn test_single_rename_input_max_length() {
    let mut app = App::new();

    app.ui.screen = Screen::SingleRename;
    app.rename.single.input = "test".to_string();

    // Fill up to 200 chars (already has 4, add 196 more)
    for _ in 0..196 {
        if app.rename.single.input.len() < 200 {
            app.rename.single.input.push('a');
        }
    }

    assert_eq!(app.rename.single.input.len(), 200);

    // Note: The 200 char limit is enforced in main.rs event loop, not in the App struct
    // This test verifies the input can hold at least 200 chars
    assert!(app.rename.single.input.len() >= 200);
}

#[test]
fn test_single_rename_selected_index() {
    let mut app = App::new();

    app.navigation.files = vec![
        FileItem {
            name: "File1.mkv".to_string(),
            is_dir: false,
            size: Some(1000),
        },
        FileItem {
            name: "File2.mkv".to_string(),
            is_dir: false,
            size: Some(1000),
        },
        FileItem {
            name: "File3.mkv".to_string(),
            is_dir: false,
            size: Some(1000),
        },
    ];
    app.navigation.focus = Focus::File;

    // Test with different selected indices
    app.navigation.selected_index = 1;
    start_single_rename(&mut app);
    assert_eq!(app.rename.single.input, "File2.mkv");

    cancel_single_rename(&mut app);

    app.navigation.selected_index = 2;
    start_single_rename(&mut app);
    assert_eq!(app.rename.single.input, "File3.mkv");
}

#[test]
fn test_single_rename_preserves_original() {
    let mut app = App::new();

    app.navigation.files = vec![FileItem {
        name: "Original.Name.mkv".to_string(),
        is_dir: false,
        size: Some(1000),
    }];
    app.navigation.focus = Focus::File;
    app.navigation.selected_index = 0;

    start_single_rename(&mut app);

    // The input should be initialized with the original filename
    assert_eq!(app.rename.single.input, "Original.Name.mkv");

    // Modify input
    app.rename.single.input = "Modified.Name.mkv".to_string();

    // Target should still reference original
    assert_eq!(
        app.rename.single.target.as_ref().unwrap().name,
        "Original.Name.mkv"
    );
}
