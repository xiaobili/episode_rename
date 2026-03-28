use openlist_tui::app::{App, Screen, RegexFocus};
use openlist_tui::api::types::FileItem;
use openlist_tui::update::*;

#[test]
fn test_start_regex_mode_initializes_state() {
    let mut app = App::new();

    // Add some mock files
    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_regex_mode(&mut app);

    assert!(matches!(app.ui.screen, Screen::RegexRename));
    assert_eq!(app.rename.regex.focus, RegexFocus::Find);
    assert!(app.rename.regex.find.is_empty());
    assert!(app.rename.regex.replace.is_empty());
    assert!(app.rename.regex.preview.is_empty());
    assert!(app.rename.regex.results.is_empty());
    assert!(!app.rename.regex.finished);
    assert!(app.rename.regex.error.is_none());
}

#[test]
fn test_start_regex_mode_empty_files() {
    let mut app = App::new();

    start_regex_mode(&mut app);

    assert!(!matches!(app.ui.screen, Screen::RegexRename));
}

#[test]
fn test_regex_validation_invalid_pattern() {
    let mut app = App::new();
    app.rename.regex.find = "[invalid".to_string(); // Invalid regex - unclosed bracket
    app.rename.regex.replace = "replacement".to_string();

    submit_regex(&mut app);

    assert!(app.rename.regex.error.is_some());
    assert!(app.rename.regex.error.as_ref().unwrap().contains("正则表达式无效"));
}

#[test]
fn test_regex_validation_valid_pattern() {
    let mut app = App::new();
    app.rename.regex.find = r"\d+".to_string(); // Valid regex - matches digits
    app.rename.regex.replace = "X".to_string();

    submit_regex(&mut app);

    assert!(app.rename.regex.error.is_none());
}

#[test]
fn test_generate_regex_preview_basic() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "Episode.01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Episode.02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Episode.03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // In Rust's regex crate, use ${1} for capture groups
    app.rename.regex.find = r"Episode\.(\d+)".to_string();
    app.rename.regex.replace = "Show.S01E${1}".to_string();

    generate_regex_preview(&mut app);

    assert_eq!(app.rename.regex.preview.len(), 3);
    assert_eq!(app.rename.regex.preview[0], ("Episode.01.mkv".to_string(), "Show.S01E01.mkv".to_string()));
    assert_eq!(app.rename.regex.preview[1], ("Episode.02.mkv".to_string(), "Show.S01E02.mkv".to_string()));
    assert_eq!(app.rename.regex.preview[2], ("Episode.03.mkv".to_string(), "Show.S01E03.mkv".to_string()));
}

#[test]
fn test_generate_regex_preview_capture_groups() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "MyShow.S01E01.1080p.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "MyShow.S01E02.1080p.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // Capture show name, season, episode and reformat
    // In Rust's regex crate, use ${1}, ${2} etc. for capture groups followed by alphanumerics
    app.rename.regex.find = r"([^.]+)\.S(\d+)E(\d+)\.\d+p".to_string();
    app.rename.regex.replace = "${1}_S${2}E${3}".to_string();

    generate_regex_preview(&mut app);

    assert_eq!(app.rename.regex.preview.len(), 2);
    assert!(app.rename.regex.preview[0].1.contains("_S01E01"));
}

#[test]
fn test_generate_regex_preview_no_matches() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "Video1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Video2.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.regex.find = r"NONEXISTENT".to_string();
    app.rename.regex.replace = "replacement".to_string();

    generate_regex_preview(&mut app);

    // No matches means no preview items (files unchanged)
    assert!(app.rename.regex.preview.is_empty());
}

#[test]
fn test_execute_regex_rename() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "old_01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old_02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old_03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // In Rust's regex crate, use ${1} for capture groups
    app.rename.regex.find = r"old_(\d+)".to_string();
    app.rename.regex.replace = "new_${1}".to_string();

    let results = execute_regex_rename(&mut app);

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("old_01.mkv".to_string(), "new_01.mkv".to_string(), true));
    assert_eq!(results[1], ("old_02.mkv".to_string(), "new_02.mkv".to_string(), true));
    assert_eq!(results[2], ("old_03.mkv".to_string(), "new_03.mkv".to_string(), true));

    assert!(app.rename.regex.finished);
    assert!(!matches!(app.ui.screen, Screen::RegexRename));
}

#[test]
fn test_execute_regex_rename_with_capture_groups() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // In Rust's regex crate, use ${1}, ${2} etc. for capture groups followed by alphanumerics
    app.rename.regex.find = r"([^.]+)\.S(\d+)E(\d+)".to_string();
    app.rename.regex.replace = "${1}_S${2}E${3}".to_string();

    let results = execute_regex_rename(&mut app);

    assert_eq!(results.len(), 2);
    // The pattern should produce: Show_S01E01.mkv
    assert!(results[0].1.contains("_S01E01"));
    assert!(results[1].1.contains("_S01E02"));
}

#[test]
fn test_cancel_regex() {
    let mut app = App::new();
    app.ui.screen = Screen::RegexRename;
    app.rename.regex.find = "test".to_string();
    app.rename.regex.replace = "replacement".to_string();
    app.rename.regex.focus = RegexFocus::Replace;
    app.rename.regex.preview = vec![("old".to_string(), "new".to_string())];
    app.rename.regex.results = vec![("old".to_string(), "new".to_string(), true)];
    app.rename.regex.error = Some("error".to_string());

    cancel_regex(&mut app);

    assert!(!matches!(app.ui.screen, Screen::RegexRename));
    assert!(app.rename.regex.find.is_empty());
    assert!(app.rename.regex.replace.is_empty());
    assert_eq!(app.rename.regex.focus, RegexFocus::Find);
    assert!(app.rename.regex.preview.is_empty());
    assert!(app.rename.regex.results.is_empty());
    assert!(!app.rename.regex.finished);
    assert!(app.rename.regex.error.is_none());
}

#[test]
fn test_toggle_regex_focus() {
    let mut app = App::new();

    app.rename.regex.focus = RegexFocus::Find;
    toggle_regex_focus(&mut app);
    assert_eq!(app.rename.regex.focus, RegexFocus::Replace);

    toggle_regex_focus(&mut app);
    assert_eq!(app.rename.regex.focus, RegexFocus::Find);
}

#[test]
fn test_take_regex_rename_results_clears() {
    let mut app = App::new();
    app.rename.regex.results = vec![
        ("Old.mkv".to_string(), "New.mkv".to_string(), true),
        ("Old2.mkv".to_string(), "New2.mkv".to_string(), true),
    ];

    let results = app.take_regex_rename_results();

    assert_eq!(results.len(), 2);
    assert!(app.rename.regex.results.is_empty());
}

#[test]
fn test_has_regex_preview() {
    let mut app = App::new();

    // No preview
    assert!(!app.has_regex_preview());

    // With preview
    app.rename.regex.preview = vec![("old".to_string(), "new".to_string())];
    assert!(app.has_regex_preview());

    // With error
    app.rename.regex.error = Some("error".to_string());
    assert!(!app.has_regex_preview());
}

#[test]
fn test_regex_simple_find_replace() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "test_file_1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "test_file_2.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.regex.find = "test".to_string();
    app.rename.regex.replace = "prod".to_string();

    generate_regex_preview(&mut app);

    assert_eq!(app.rename.regex.preview.len(), 2);
    assert_eq!(app.rename.regex.preview[0], ("test_file_1.mkv".to_string(), "prod_file_1.mkv".to_string()));
    assert_eq!(app.rename.regex.preview[1], ("test_file_2.mkv".to_string(), "prod_file_2.mkv".to_string()));
}

#[test]
fn test_regex_remove_extension_pattern() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "Video.2023.1080p.mp4".to_string(), is_dir: false, size: Some(1000) },
    ];
    // Remove year and resolution
    app.rename.regex.find = r"\.\d{4}\.\d+p".to_string();
    app.rename.regex.replace = "".to_string();

    generate_regex_preview(&mut app);

    assert_eq!(app.rename.regex.preview.len(), 1);
    assert_eq!(app.rename.regex.preview[0], ("Video.2023.1080p.mp4".to_string(), "Video.mp4".to_string()));
}

#[test]
fn test_regex_state_transitions() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "A.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // Initial state
    assert!(!matches!(app.ui.screen, Screen::RegexRename));

    // Start regex mode
    start_regex_mode(&mut app);
    assert!(matches!(app.ui.screen, Screen::RegexRename));

    // Input find pattern
    app.rename.regex.find = "old".to_string();
    app.rename.regex.replace = "new".to_string();

    // Submit to generate preview
    submit_regex(&mut app);
    assert!(app.rename.regex.error.is_none());

    // Cancel
    cancel_regex(&mut app);
    assert!(!matches!(app.ui.screen, Screen::RegexRename));
    assert!(app.rename.regex.find.is_empty());
}

#[test]
fn test_regex_multiple_capture_groups() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "Series.S02E15.720p.HDTV.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // Reformat: extract components and rearrange
    // In Rust's regex crate, use ${1}, ${2} etc. for capture groups followed by alphanumerics
    app.rename.regex.find = r"([^.]+)\.S(\d+)E(\d+)\.([^\.]+)\.([^\.]+)".to_string();
    app.rename.regex.replace = "${1} - S${2}E${3}".to_string();

    generate_regex_preview(&mut app);

    assert_eq!(app.rename.regex.preview.len(), 1);
    assert!(app.rename.regex.preview[0].1.contains(" - S02E15"));
}

#[test]
fn test_regex_invalid_then_valid() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "test.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // First try invalid regex
    app.rename.regex.find = "[invalid".to_string();
    submit_regex(&mut app);
    assert!(app.rename.regex.error.is_some());

    // Then fix to valid regex
    app.rename.regex.find = "test".to_string();
    submit_regex(&mut app);
    assert!(app.rename.regex.error.is_none());
    assert!(!app.rename.regex.preview.is_empty());
}
