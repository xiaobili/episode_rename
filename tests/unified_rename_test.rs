use openlist_tui::app::{App, Screen, UnifiedFocus};
use openlist_tui::api::types::FileItem;
use openlist_tui::update::*;

#[test]
fn test_start_unified_mode_initializes_state() {
    let mut app = App::new();

    // Add some mock files
    app.navigation.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    start_unified_mode(&mut app);

    assert!(matches!(app.ui.screen, Screen::UnifiedRename));
    assert_eq!(app.rename.unified.focus, UnifiedFocus::ShowName);
    assert_eq!(app.rename.unified.season, "1");
    assert_eq!(app.rename.unified.start_episode, "1");
    assert_eq!(app.rename.unified.pattern, "{title}.S{season}E{episode}");
    assert!(!app.rename.unified.preview.is_empty()); // Preview is generated in start_unified_mode
    assert!(app.rename.unified.results.is_empty());
    assert!(!app.rename.unified.finished);
}

#[test]
fn test_start_unified_mode_empty_files() {
    let mut app = App::new();

    start_unified_mode(&mut app);

    assert!(!matches!(app.ui.screen, Screen::UnifiedRename));
}

#[test]
fn test_unified_naming_input_validation_empty_show_name() {
    let mut app = App::new();
    app.rename.unified.show_name.clear();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "1".to_string();

    let result = validate_unified_inputs(&app);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("剧集名称不能为空"));
}

#[test]
fn test_unified_naming_input_validation_empty_season() {
    let mut app = App::new();
    app.rename.unified.show_name = "Test Show".to_string();
    app.rename.unified.season.clear();
    app.rename.unified.start_episode = "1".to_string();

    let result = validate_unified_inputs(&app);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("季数不能为空"));
}

#[test]
fn test_unified_naming_input_validation_non_numeric_season() {
    let mut app = App::new();
    app.rename.unified.show_name = "Test Show".to_string();
    app.rename.unified.season = "abc".to_string();
    app.rename.unified.start_episode = "1".to_string();

    let result = validate_unified_inputs(&app);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("季数必须是数字"));
}

#[test]
fn test_unified_naming_input_validation_empty_episode() {
    let mut app = App::new();
    app.rename.unified.show_name = "Test Show".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode.clear();

    let result = validate_unified_inputs(&app);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("起始集数不能为空"));
}

#[test]
fn test_unified_naming_input_validation_non_numeric_episode() {
    let mut app = App::new();
    app.rename.unified.show_name = "Test Show".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "abc".to_string();

    let result = validate_unified_inputs(&app);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("起始集数必须是数字"));
}

#[test]
fn test_unified_naming_input_validation_success() {
    let mut app = App::new();
    app.rename.unified.show_name = "Test Show".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "5".to_string();

    let result = validate_unified_inputs(&app);
    assert!(result.is_ok());
}

#[test]
fn test_generate_unified_preview() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "Video1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Video2.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Video3.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "My Show".to_string();
    app.rename.unified.season = "2".to_string();
    app.rename.unified.start_episode = "3".to_string();
    app.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();

    generate_unified_preview(&mut app);

    assert_eq!(app.rename.unified.preview.len(), 3);
    // First file: My Show.S02E03.mkv
    assert!(app.rename.unified.preview[0].contains("My Show.S02E03"));
    // Second file: My Show.S02E04.mkv
    assert!(app.rename.unified.preview[1].contains("My Show.S02E04"));
    // Third file: My Show.S02E05.mkv
    assert!(app.rename.unified.preview[2].contains("My Show.S02E05"));
}

#[test]
fn test_generate_unified_preview_custom_pattern() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "ep1.mp4".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "ep2.mp4".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "Series".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "1".to_string();
    app.rename.unified.pattern = "{title}_{season}x{episode}".to_string();

    generate_unified_preview(&mut app);

    assert_eq!(app.rename.unified.preview.len(), 2);
    // First file: Series_01x01.mp4
    assert!(app.rename.unified.preview[0].contains("Series_01x01"));
    // Second file: Series_01x02.mp4
    assert!(app.rename.unified.preview[1].contains("Series_01x02"));
}

#[test]
fn test_generate_unified_preview_many_files() {
    let mut app = App::new();
    // Add 10 mock files
    for i in 1..=10 {
        app.navigation.files.push(FileItem {
            name: format!("video{}.mkv", i),
            is_dir: false,
            size: Some(1000),
        });
    }
    app.rename.unified.show_name = "Show".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "1".to_string();
    app.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();

    generate_unified_preview(&mut app);

    // Should show first 5 files plus "... 还有 5 个文件"
    assert_eq!(app.rename.unified.preview.len(), 6);
    assert!(app.rename.unified.preview[5].contains("还有"));
}

#[test]
fn test_toggle_unified_focus() {
    let mut app = App::new();

    // Start at ShowName
    app.rename.unified.focus = UnifiedFocus::ShowName;
    toggle_unified_focus(&mut app);
    assert_eq!(app.rename.unified.focus, UnifiedFocus::Season);

    toggle_unified_focus(&mut app);
    assert_eq!(app.rename.unified.focus, UnifiedFocus::StartEpisode);

    toggle_unified_focus(&mut app);
    assert_eq!(app.rename.unified.focus, UnifiedFocus::Pattern);

    toggle_unified_focus(&mut app);
    assert_eq!(app.rename.unified.focus, UnifiedFocus::ShowName);
}

#[test]
fn test_cancel_unified() {
    let mut app = App::new();
    app.ui.screen = Screen::UnifiedRename;
    app.rename.unified.show_name = "Test".to_string();
    app.rename.unified.season = "2".to_string();
    app.rename.unified.start_episode = "3".to_string();
    app.rename.unified.pattern = "custom".to_string();
    app.rename.unified.focus = UnifiedFocus::Pattern;
    app.rename.unified.preview = vec!["preview".to_string()];
    app.rename.unified.results = vec![("old".to_string(), "new".to_string(), true)];

    cancel_unified(&mut app);

    assert!(!matches!(app.ui.screen, Screen::UnifiedRename));
    assert!(app.rename.unified.show_name.is_empty());
    assert!(app.rename.unified.season.is_empty());
    assert!(app.rename.unified.start_episode.is_empty());
    assert_eq!(app.rename.unified.pattern, "{title}.S{season}E{episode}");
    assert_eq!(app.rename.unified.focus, UnifiedFocus::ShowName);
    assert!(app.rename.unified.preview.is_empty());
    assert!(app.rename.unified.results.is_empty());
    assert!(!app.rename.unified.finished);
}

#[test]
fn test_execute_unified_rename() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "old1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old2.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old3.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "NewShow".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "1".to_string();
    app.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();

    let results = execute_unified_rename(&mut app);

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("old1.mkv".to_string(), "NewShow.S01E01.mkv".to_string(), true));
    assert_eq!(results[1], ("old2.mkv".to_string(), "NewShow.S01E02.mkv".to_string(), true));
    assert_eq!(results[2], ("old3.mkv".to_string(), "NewShow.S01E03.mkv".to_string(), true));

    assert!(app.rename.unified.finished);
    assert!(!matches!(app.ui.screen, Screen::UnifiedRename));
}

#[test]
fn test_execute_unified_rename_custom_start_episode() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "vid1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "vid2.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "Series".to_string();
    app.rename.unified.season = "3".to_string();
    app.rename.unified.start_episode = "10".to_string();
    app.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();

    let results = execute_unified_rename(&mut app);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], ("vid1.mkv".to_string(), "Series.S03E10.mkv".to_string(), true));
    assert_eq!(results[1], ("vid2.mkv".to_string(), "Series.S03E11.mkv".to_string(), true));
}

#[test]
fn test_take_unified_rename_results_clears() {
    let mut app = App::new();
    app.rename.unified.results = vec![
        ("Old.mkv".to_string(), "New.mkv".to_string(), true),
        ("Old2.mkv".to_string(), "New2.mkv".to_string(), true),
    ];

    let results = app.take_unified_rename_results();

    assert_eq!(results.len(), 2);
    assert!(app.rename.unified.results.is_empty());
}

#[test]
fn test_unified_naming_zero_padding() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "ep1.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "Test".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "1".to_string();
    app.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();

    generate_unified_preview(&mut app);

    // Should have zero-padded season and episode
    assert!(app.rename.unified.preview[0].contains("S01E01"));
}

#[test]
fn test_unified_naming_double_digit_season_episode() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "ep1.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "Test".to_string();
    app.rename.unified.season = "12".to_string();
    app.rename.unified.start_episode = "15".to_string();
    app.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();

    generate_unified_preview(&mut app);

    // Should have correct double digit formatting
    assert!(app.rename.unified.preview[0].contains("S12E15"));
}

#[test]
fn test_unified_naming_pattern_placeholders() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "video.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.rename.unified.show_name = "MyShow".to_string();
    app.rename.unified.season = "1".to_string();
    app.rename.unified.start_episode = "1".to_string();

    // Test with different pattern formats
    app.rename.unified.pattern = "{title}_Season{season}_Episode{episode}".to_string();
    generate_unified_preview(&mut app);
    assert!(app.rename.unified.preview[0].contains("MyShow_Season01_Episode01"));
}

#[test]
fn test_unified_naming_state_transitions() {
    let mut app = App::new();
    app.navigation.files = vec![
        FileItem { name: "A.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "B.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // Initial state
    assert!(!matches!(app.ui.screen, Screen::UnifiedRename));
    assert_eq!(app.rename.unified.show_name, "");

    // Start unified mode
    start_unified_mode(&mut app);
    assert!(matches!(app.ui.screen, Screen::UnifiedRename));
    assert_eq!(app.rename.unified.focus, UnifiedFocus::ShowName);

    // Input show name
    app.rename.unified.show_name = "Test Show".to_string();
    generate_unified_preview(&mut app);
    assert!(!app.rename.unified.preview.is_empty());

    // Cancel
    cancel_unified(&mut app);
    assert!(!matches!(app.ui.screen, Screen::UnifiedRename));
    assert!(app.rename.unified.show_name.is_empty());
}
