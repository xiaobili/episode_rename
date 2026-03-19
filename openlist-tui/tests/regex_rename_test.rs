use openlist_tui::app::App;
use openlist_tui::api::types::FileItem;

#[test]
fn test_start_regex_mode_initializes_state() {
    let mut app = App::new();

    // Add some mock files
    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_regex_mode();

    assert!(app.show_regex_input);
    assert_eq!(app.regex_focus, openlist_tui::app::RegexFocus::Find);
    assert!(app.regex_find.is_empty());
    assert!(app.regex_replace.is_empty());
    assert!(app.regex_preview.is_empty());
    assert!(app.regex_rename_results.is_empty());
    assert!(!app.regex_rename_finished);
    assert!(app.regex_error.is_none());
}

#[test]
fn test_start_regex_mode_empty_files() {
    let mut app = App::new();

    app.start_regex_mode();

    assert!(!app.show_regex_input);
}

#[test]
fn test_regex_validation_invalid_pattern() {
    let mut app = App::new();
    app.regex_find = "[invalid".to_string(); // Invalid regex - unclosed bracket
    app.regex_replace = "replacement".to_string();

    app.submit_regex();

    assert!(app.regex_error.is_some());
    assert!(app.regex_error.as_ref().unwrap().contains("正则表达式无效"));
}

#[test]
fn test_regex_validation_valid_pattern() {
    let mut app = App::new();
    app.regex_find = r"\\d+".to_string(); // Valid regex - matches digits
    app.regex_replace = "X".to_string();

    app.submit_regex();

    assert!(app.regex_error.is_none());
}

#[test]
fn test_generate_regex_preview_basic() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "Episode.01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Episode.02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Episode.03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // In Rust's regex crate, use ${1} for capture groups
    app.regex_find = r"Episode\.(\d+)".to_string();
    app.regex_replace = "Show.S01E${1}".to_string();

    app.generate_regex_preview();

    assert_eq!(app.regex_preview.len(), 3);
    assert_eq!(app.regex_preview[0], ("Episode.01.mkv".to_string(), "Show.S01E01.mkv".to_string()));
    assert_eq!(app.regex_preview[1], ("Episode.02.mkv".to_string(), "Show.S01E02.mkv".to_string()));
    assert_eq!(app.regex_preview[2], ("Episode.03.mkv".to_string(), "Show.S01E03.mkv".to_string()));
}

#[test]
fn test_generate_regex_preview_capture_groups() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "MyShow.S01E01.1080p.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "MyShow.S01E02.1080p.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // Capture show name, season, episode and reformat
    // In Rust's regex crate, use ${1}, ${2} etc. for capture groups followed by alphanumerics
    app.regex_find = r"([^.]+)\.S(\d+)E(\d+)\.\d+p".to_string();
    app.regex_replace = "${1}_S${2}E${3}".to_string();

    app.generate_regex_preview();

    assert_eq!(app.regex_preview.len(), 2);
    assert!(app.regex_preview[0].1.contains("_S01E01"));
}

#[test]
fn test_generate_regex_preview_no_matches() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "Video1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Video2.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.regex_find = r"NONEXISTENT".to_string();
    app.regex_replace = "replacement".to_string();

    app.generate_regex_preview();

    // No matches means no preview items (files unchanged)
    assert!(app.regex_preview.is_empty());
}

#[test]
fn test_execute_regex_rename() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "old_01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old_02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old_03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // In Rust's regex crate, use ${1} for capture groups
    app.regex_find = r"old_(\d+)".to_string();
    app.regex_replace = "new_${1}".to_string();

    let results = app.execute_regex_rename();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("old_01.mkv".to_string(), "new_01.mkv".to_string(), true));
    assert_eq!(results[1], ("old_02.mkv".to_string(), "new_02.mkv".to_string(), true));
    assert_eq!(results[2], ("old_03.mkv".to_string(), "new_03.mkv".to_string(), true));

    assert!(app.regex_rename_finished);
    assert!(!app.show_regex_input);
}

#[test]
fn test_execute_regex_rename_with_capture_groups() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // In Rust's regex crate, use ${1}, ${2} etc. for capture groups followed by alphanumerics
    app.regex_find = r"([^.]+)\.S(\d+)E(\d+)".to_string();
    app.regex_replace = "${1}_S${2}E${3}".to_string();

    let results = app.execute_regex_rename();

    assert_eq!(results.len(), 2);
    // The pattern should produce: Show_S01E01.mkv
    assert!(results[0].1.contains("_S01E01"));
    assert!(results[1].1.contains("_S01E02"));
}

#[test]
fn test_cancel_regex() {
    let mut app = App::new();
    app.show_regex_input = true;
    app.regex_find = "test".to_string();
    app.regex_replace = "replacement".to_string();
    app.regex_focus = openlist_tui::app::RegexFocus::Replace;
    app.regex_preview = vec![("old".to_string(), "new".to_string())];
    app.regex_rename_results = vec![("old".to_string(), "new".to_string(), true)];
    app.regex_error = Some("error".to_string());

    app.cancel_regex();

    assert!(!app.show_regex_input);
    assert!(app.regex_find.is_empty());
    assert!(app.regex_replace.is_empty());
    assert_eq!(app.regex_focus, openlist_tui::app::RegexFocus::Find);
    assert!(app.regex_preview.is_empty());
    assert!(app.regex_rename_results.is_empty());
    assert!(!app.regex_rename_finished);
    assert!(app.regex_error.is_none());
}

#[test]
fn test_toggle_regex_focus() {
    let mut app = App::new();

    app.regex_focus = openlist_tui::app::RegexFocus::Find;
    app.toggle_regex_focus();
    assert_eq!(app.regex_focus, openlist_tui::app::RegexFocus::Replace);

    app.toggle_regex_focus();
    assert_eq!(app.regex_focus, openlist_tui::app::RegexFocus::Find);
}

#[test]
fn test_take_regex_rename_results_clears() {
    let mut app = App::new();
    app.regex_rename_results = vec![
        ("Old.mkv".to_string(), "New.mkv".to_string(), true),
        ("Old2.mkv".to_string(), "New2.mkv".to_string(), true),
    ];

    let results = app.take_regex_rename_results();

    assert_eq!(results.len(), 2);
    assert!(app.regex_rename_results.is_empty());
}

#[test]
fn test_has_regex_preview() {
    let mut app = App::new();

    // No preview
    assert!(!app.has_regex_preview());

    // With preview
    app.regex_preview = vec![("old".to_string(), "new".to_string())];
    assert!(app.has_regex_preview());

    // With error
    app.regex_error = Some("error".to_string());
    assert!(!app.has_regex_preview());
}

#[test]
fn test_regex_simple_find_replace() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "test_file_1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "test_file_2.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.regex_find = "test".to_string();
    app.regex_replace = "prod".to_string();

    app.generate_regex_preview();

    assert_eq!(app.regex_preview.len(), 2);
    assert_eq!(app.regex_preview[0], ("test_file_1.mkv".to_string(), "prod_file_1.mkv".to_string()));
    assert_eq!(app.regex_preview[1], ("test_file_2.mkv".to_string(), "prod_file_2.mkv".to_string()));
}

#[test]
fn test_regex_remove_extension_pattern() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "Video.2023.1080p.mp4".to_string(), is_dir: false, size: Some(1000) },
    ];
    // Remove year and resolution
    app.regex_find = r"\.\d{4}\.\d+p".to_string();
    app.regex_replace = "".to_string();

    app.generate_regex_preview();

    assert_eq!(app.regex_preview.len(), 1);
    assert_eq!(app.regex_preview[0], ("Video.2023.1080p.mp4".to_string(), "Video.mp4".to_string()));
}

#[test]
fn test_regex_state_transitions() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "A.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // Initial state
    assert!(!app.show_regex_input);

    // Start regex mode
    app.start_regex_mode();
    assert!(app.show_regex_input);

    // Input find pattern
    app.regex_find = "old".to_string();
    app.regex_replace = "new".to_string();

    // Submit to generate preview
    app.submit_regex();
    assert!(app.regex_error.is_none());

    // Cancel
    app.cancel_regex();
    assert!(!app.show_regex_input);
    assert!(app.regex_find.is_empty());
}

#[test]
fn test_regex_multiple_capture_groups() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "Series.S02E15.720p.HDTV.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    // Reformat: extract components and rearrange
    // In Rust's regex crate, use ${1}, ${2} etc. for capture groups followed by alphanumerics
    app.regex_find = r"([^.]+)\.S(\d+)E(\d+)\.([^\.]+)\.([^\.]+)".to_string();
    app.regex_replace = "${1} - S${2}E${3}".to_string();

    app.generate_regex_preview();

    assert_eq!(app.regex_preview.len(), 1);
    assert!(app.regex_preview[0].1.contains(" - S02E15"));
}

#[test]
fn test_regex_invalid_then_valid() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "test.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // First try invalid regex
    app.regex_find = "[invalid".to_string();
    app.submit_regex();
    assert!(app.regex_error.is_some());

    // Then fix to valid regex
    app.regex_find = "test".to_string();
    app.submit_regex();
    assert!(app.regex_error.is_none());
    assert!(!app.regex_preview.is_empty());
}
