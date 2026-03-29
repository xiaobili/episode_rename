use openlist_tui::api::types::FileItem;
use openlist_tui::app::{App, Focus, Screen};
use openlist_tui::update::*;
use openlist_tui::validate::validate_folder_name;

// === FOLD-01: Keyboard shortcut tests ===

#[test]
fn test_start_folder_rename_shortcut() {
    let mut app = App::new();

    app.navigation.directories = vec![FileItem {
        name: "Folder1".to_string(),
        is_dir: true,
        size: None,
    }];
    app.navigation.focus = Focus::Directory;
    app.navigation.selected_index = 0;
    app.navigation.current_path = "/test".to_string();

    start_folder_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::FolderRename));
    assert!(app.rename.folder.target.is_some());
    assert_eq!(app.rename.folder.input, "Folder1");
}

#[test]
fn test_start_folder_rename_wrong_focus() {
    let mut app = App::new();

    app.navigation.directories = vec![FileItem {
        name: "Folder1".to_string(),
        is_dir: true,
        size: None,
    }];
    app.navigation.focus = Focus::File; // Wrong focus
    app.navigation.selected_index = 0;

    start_folder_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::FolderRename));
    assert!(app.rename.folder.target.is_none());
}

#[test]
fn test_start_folder_rename_empty_directories() {
    let mut app = App::new();

    app.navigation.directories = vec![];
    app.navigation.focus = Focus::Directory;
    app.navigation.selected_index = 0;

    start_folder_rename(&mut app);

    assert!(!matches!(app.ui.screen, Screen::FolderRename));
}

// === FOLD-02: Input handling tests ===

#[test]
fn test_folder_rename_input() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.input = String::new();

    input_folder_rename_char(&mut app, 'a');
    input_folder_rename_char(&mut app, 'b');
    input_folder_rename_char(&mut app, 'c');

    assert_eq!(app.rename.folder.input, "abc");
}

#[test]
fn test_folder_rename_input_max_length() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.input = "a".repeat(255);

    input_folder_rename_char(&mut app, 'b'); // Should be ignored

    assert_eq!(app.rename.folder.input.len(), 255);
}

#[test]
fn test_folder_rename_backspace() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.input = "test".to_string();

    delete_last_folder_rename_char(&mut app);

    assert_eq!(app.rename.folder.input, "tes");
}

// === FOLD-03: Cancel tests ===

#[test]
fn test_cancel_folder_rename() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.input = "test".to_string();
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.validation_error = Some("error".to_string());

    cancel_folder_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::Normal));
    assert!(app.rename.folder.input.is_empty());
    assert!(app.rename.folder.target.is_none());
    assert!(app.rename.folder.validation_error.is_none());
}

// === FOLD-04: Validation tests ===

#[test]
fn test_validate_empty_folder_name() {
    let existing = vec![];
    let result = validate_folder_name("", &existing);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "文件夹名称不能为空");
}

#[test]
fn test_validate_folder_name_length() {
    let existing = vec![];
    let long_name = "a".repeat(256);
    let result = validate_folder_name(&long_name, &existing);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "文件夹名称不能超过255个字符");
}

#[test]
fn test_validate_reserved_names() {
    let existing = vec![];

    // Unix reserved
    let result = validate_folder_name(".", &existing);
    assert!(result.is_some());

    let result = validate_folder_name("..", &existing);
    assert!(result.is_some());

    // Windows reserved
    let result = validate_folder_name("CON", &existing);
    assert!(result.is_some());

    let result = validate_folder_name("aux", &existing); // case insensitive
    assert!(result.is_some());

    let result = validate_folder_name("COM1", &existing);
    assert!(result.is_some());

    let result = validate_folder_name("lpt9", &existing);
    assert!(result.is_some());
}

#[test]
fn test_validate_invalid_chars() {
    let existing = vec![];

    let invalid_names = [
        "test/file",
        "test\\file",
        "test:file",
        "test*file",
        "test?file",
        "test\"file",
        "test<file",
        "test>file",
        "test|file",
    ];

    for name in invalid_names {
        let result = validate_folder_name(name, &existing);
        assert!(result.is_some(), "Expected error for: {}", name);
    }
}

#[test]
fn test_validate_duplicate_names() {
    let existing = vec![FileItem {
        name: "ExistingFolder".to_string(),
        is_dir: true,
        size: None,
    }];

    let result = validate_folder_name("ExistingFolder", &existing);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "已存在同名文件夹");
}

#[test]
fn test_validate_valid_folder_name() {
    let existing = vec![];

    let result = validate_folder_name("ValidFolder", &existing);
    assert!(result.is_none());

    let result = validate_folder_name("文件夹", &existing);
    assert!(result.is_none());

    let result = validate_folder_name("Folder 123", &existing);
    assert!(result.is_none());
}

// === FOLD-05, FOLD-06, FOLD-07, FOLD-08: Integration tests ===

#[test]
fn test_submit_folder_rename() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.input = "NewFolder".to_string();

    submit_folder_rename(&mut app);

    // Should clear validation error since input is valid
    assert!(app.rename.folder.validation_error.is_none());
}

#[test]
fn test_submit_folder_rename_validation_error() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.input = "".to_string(); // Empty name

    submit_folder_rename(&mut app);

    // Should have validation error
    assert!(app.rename.folder.validation_error.is_some());
    assert_eq!(
        app.rename.folder.validation_error.unwrap(),
        "文件夹名称不能为空"
    );
}

// === State transition tests ===

#[test]
fn test_folder_rename_state_transitions() {
    let mut app = App::new();

    // Initial state
    assert!(!matches!(app.ui.screen, Screen::FolderRename));
    assert!(app.rename.folder.target.is_none());
    assert!(app.rename.folder.validation_error.is_none());

    // Setup
    app.navigation.directories = vec![FileItem {
        name: "TestFolder".to_string(),
        is_dir: true,
        size: None,
    }];
    app.navigation.focus = Focus::Directory;
    app.navigation.selected_index = 0;
    app.navigation.current_path = "/test".to_string();

    // Start rename
    start_folder_rename(&mut app);
    assert!(matches!(app.ui.screen, Screen::FolderRename));
    assert!(app.rename.folder.target.is_some());

    // Input new name
    app.rename.folder.input.clear();
    input_folder_rename_char(&mut app, 'N');
    input_folder_rename_char(&mut app, 'e');
    input_folder_rename_char(&mut app, 'w');
    assert_eq!(app.rename.folder.input, "New");

    // Cancel
    cancel_folder_rename(&mut app);
    assert!(matches!(app.ui.screen, Screen::Normal));
    assert!(app.rename.folder.target.is_none());
}

#[test]
fn test_folder_rename_index_with_parent_entry() {
    let mut app = App::new();

    // When current_path != "/", there's a ".." entry at index 0
    app.navigation.directories = vec![
        FileItem {
            name: "Folder1".to_string(),
            is_dir: true,
            size: None,
        },
        FileItem {
            name: "Folder2".to_string(),
            is_dir: true,
            size: None,
        },
    ];
    app.navigation.focus = Focus::Directory;
    app.navigation.current_path = "/parent".to_string();
    app.navigation.selected_index = 1; // Points to ".." entry, should map to Folder1

    start_folder_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::FolderRename));
    assert_eq!(app.rename.folder.input, "Folder1");
}

#[test]
fn test_folder_rename_index_root_path() {
    let mut app = App::new();

    // When current_path == "/", no ".." entry
    app.navigation.directories = vec![
        FileItem {
            name: "Folder1".to_string(),
            is_dir: true,
            size: None,
        },
        FileItem {
            name: "Folder2".to_string(),
            is_dir: true,
            size: None,
        },
    ];
    app.navigation.focus = Focus::Directory;
    app.navigation.current_path = "/".to_string();
    app.navigation.selected_index = 1; // Directly points to Folder2

    start_folder_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::FolderRename));
    assert_eq!(app.rename.folder.input, "Folder2");
}

// === FOLD-05, FOLD-06, FOLD-07: Additional submit validation tests ===

#[test]
fn test_submit_folder_rename_validates_duplicate() {
    let mut app = App::new();

    app.navigation.directories = vec![FileItem {
        name: "ExistingFolder".to_string(),
        is_dir: true,
        size: None,
    }];
    app.ui.screen = Screen::FolderRename;
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.input = "ExistingFolder".to_string(); // Duplicate name

    submit_folder_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::FolderRename));
    assert!(app.rename.folder.validation_error.is_some());
    assert!(app.rename.folder.validation_error.unwrap().contains("同名"));
}

#[test]
fn test_submit_folder_rename_validates_reserved() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.input = "CON".to_string(); // Reserved name

    submit_folder_rename(&mut app);

    assert!(matches!(app.ui.screen, Screen::FolderRename));
    assert!(app.rename.folder.validation_error.is_some());
}

#[test]
fn test_submit_folder_rename_valid_input() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.input = "NewValidFolder".to_string();
    app.rename.folder.validation_error = Some("previous error".to_string());

    submit_folder_rename(&mut app);

    // Validation should pass and clear error
    assert!(app.rename.folder.validation_error.is_none());
}

#[test]
fn test_folder_rename_validation_error_display() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    app.rename.folder.target = Some(FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    });
    app.rename.folder.input = "a/b".to_string(); // Invalid character
    app.rename.folder.validation_error = None;

    submit_folder_rename(&mut app);

    assert!(app.rename.folder.validation_error.is_some());
    let error = app.rename.folder.validation_error.unwrap();
    assert!(error.contains("无效字符"));
}

#[test]
fn test_folder_rename_preserves_target_on_validation_error() {
    let mut app = App::new();

    app.ui.screen = Screen::FolderRename;
    let target = FileItem {
        name: "OldFolder".to_string(),
        is_dir: true,
        size: None,
    };
    app.rename.folder.target = Some(target.clone());
    app.rename.folder.input = "".to_string(); // Empty - validation error

    submit_folder_rename(&mut app);

    // Target should still be set after validation error
    assert!(app.rename.folder.target.is_some());
    assert_eq!(app.rename.folder.target.unwrap().name, "OldFolder");
}
