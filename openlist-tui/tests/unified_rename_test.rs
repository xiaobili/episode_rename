use openlist_tui::app::App;
use openlist_tui::api::types::FileItem;

#[test]
fn test_start_unified_mode_initializes_state() {
    let mut app = App::new();

    // Add some mock files
    app.files = vec![
        FileItem { name: "Show.S01E01.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E02.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Show.S01E03.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    app.start_unified_mode();

    assert!(app.show_unified_input);
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::ShowName);
    assert_eq!(app.unified_season, "1");
    assert_eq!(app.unified_start_episode, "1");
    assert_eq!(app.unified_pattern, "{title}.S{season}E{episode}");
    assert!(!app.unified_preview.is_empty()); // Preview is generated in start_unified_mode
    assert!(app.unified_rename_results.is_empty());
    assert!(!app.unified_rename_finished);
}

#[test]
fn test_start_unified_mode_empty_files() {
    let mut app = App::new();

    app.start_unified_mode();

    assert!(!app.show_unified_input);
}

#[test]
fn test_unified_naming_input_validation_empty_show_name() {
    let mut app = App::new();
    app.unified_show_name.clear();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "1".to_string();

    let result = app.validate_unified_inputs();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("剧集名称不能为空"));
}

#[test]
fn test_unified_naming_input_validation_empty_season() {
    let mut app = App::new();
    app.unified_show_name = "Test Show".to_string();
    app.unified_season.clear();
    app.unified_start_episode = "1".to_string();

    let result = app.validate_unified_inputs();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("季数不能为空"));
}

#[test]
fn test_unified_naming_input_validation_non_numeric_season() {
    let mut app = App::new();
    app.unified_show_name = "Test Show".to_string();
    app.unified_season = "abc".to_string();
    app.unified_start_episode = "1".to_string();

    let result = app.validate_unified_inputs();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("季数必须是数字"));
}

#[test]
fn test_unified_naming_input_validation_empty_episode() {
    let mut app = App::new();
    app.unified_show_name = "Test Show".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode.clear();

    let result = app.validate_unified_inputs();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("起始集数不能为空"));
}

#[test]
fn test_unified_naming_input_validation_non_numeric_episode() {
    let mut app = App::new();
    app.unified_show_name = "Test Show".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "abc".to_string();

    let result = app.validate_unified_inputs();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("起始集数必须是数字"));
}

#[test]
fn test_unified_naming_input_validation_success() {
    let mut app = App::new();
    app.unified_show_name = "Test Show".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "5".to_string();

    let result = app.validate_unified_inputs();
    assert!(result.is_ok());
}

#[test]
fn test_generate_unified_preview() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "Video1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Video2.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "Video3.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "My Show".to_string();
    app.unified_season = "2".to_string();
    app.unified_start_episode = "3".to_string();
    app.unified_pattern = "{title}.S{season}E{episode}".to_string();

    app.generate_unified_preview();

    assert_eq!(app.unified_preview.len(), 3);
    // First file: My Show.S02E03.mkv
    assert!(app.unified_preview[0].contains("My Show.S02E03"));
    // Second file: My Show.S02E04.mkv
    assert!(app.unified_preview[1].contains("My Show.S02E04"));
    // Third file: My Show.S02E05.mkv
    assert!(app.unified_preview[2].contains("My Show.S02E05"));
}

#[test]
fn test_generate_unified_preview_custom_pattern() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "ep1.mp4".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "ep2.mp4".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "Series".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "1".to_string();
    app.unified_pattern = "{title}_{season}x{episode}".to_string();

    app.generate_unified_preview();

    assert_eq!(app.unified_preview.len(), 2);
    // First file: Series_01x01.mp4
    assert!(app.unified_preview[0].contains("Series_01x01"));
    // Second file: Series_01x02.mp4
    assert!(app.unified_preview[1].contains("Series_01x02"));
}

#[test]
fn test_generate_unified_preview_many_files() {
    let mut app = App::new();
    // Add 10 mock files
    for i in 1..=10 {
        app.files.push(FileItem {
            name: format!("video{}.mkv", i),
            is_dir: false,
            size: Some(1000),
        });
    }
    app.unified_show_name = "Show".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "1".to_string();
    app.unified_pattern = "{title}.S{season}E{episode}".to_string();

    app.generate_unified_preview();

    // Should show first 5 files plus "... 还有 5 个文件"
    assert_eq!(app.unified_preview.len(), 6);
    assert!(app.unified_preview[5].contains("还有"));
}

#[test]
fn test_toggle_unified_focus() {
    let mut app = App::new();

    // Start at ShowName
    app.unified_focus = openlist_tui::app::UnifiedFocus::ShowName;
    app.toggle_unified_focus();
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::Season);

    app.toggle_unified_focus();
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::StartEpisode);

    app.toggle_unified_focus();
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::Pattern);

    app.toggle_unified_focus();
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::ShowName);
}

#[test]
fn test_cancel_unified() {
    let mut app = App::new();
    app.show_unified_input = true;
    app.unified_show_name = "Test".to_string();
    app.unified_season = "2".to_string();
    app.unified_start_episode = "3".to_string();
    app.unified_pattern = "custom".to_string();
    app.unified_focus = openlist_tui::app::UnifiedFocus::Pattern;
    app.unified_preview = vec!["preview".to_string()];
    app.unified_rename_results = vec![("old".to_string(), "new".to_string(), true)];

    app.cancel_unified();

    assert!(!app.show_unified_input);
    assert!(app.unified_show_name.is_empty());
    assert!(app.unified_season.is_empty());
    assert!(app.unified_start_episode.is_empty());
    assert_eq!(app.unified_pattern, "{title}.S{season}E{episode}");
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::ShowName);
    assert!(app.unified_preview.is_empty());
    assert!(app.unified_rename_results.is_empty());
    assert!(!app.unified_rename_finished);
}

#[test]
fn test_execute_unified_rename() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "old1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old2.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "old3.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "NewShow".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "1".to_string();
    app.unified_pattern = "{title}.S{season}E{episode}".to_string();

    let results = app.execute_unified_rename();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("old1.mkv".to_string(), "NewShow.S01E01.mkv".to_string(), true));
    assert_eq!(results[1], ("old2.mkv".to_string(), "NewShow.S01E02.mkv".to_string(), true));
    assert_eq!(results[2], ("old3.mkv".to_string(), "NewShow.S01E03.mkv".to_string(), true));

    assert!(app.unified_rename_finished);
    assert!(!app.show_unified_input);
}

#[test]
fn test_execute_unified_rename_custom_start_episode() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "vid1.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "vid2.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "Series".to_string();
    app.unified_season = "3".to_string();
    app.unified_start_episode = "10".to_string();
    app.unified_pattern = "{title}.S{season}E{episode}".to_string();

    let results = app.execute_unified_rename();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], ("vid1.mkv".to_string(), "Series.S03E10.mkv".to_string(), true));
    assert_eq!(results[1], ("vid2.mkv".to_string(), "Series.S03E11.mkv".to_string(), true));
}

#[test]
fn test_take_unified_rename_results_clears() {
    let mut app = App::new();
    app.unified_rename_results = vec![
        ("Old.mkv".to_string(), "New.mkv".to_string(), true),
        ("Old2.mkv".to_string(), "New2.mkv".to_string(), true),
    ];

    let results = app.take_unified_rename_results();

    assert_eq!(results.len(), 2);
    assert!(app.unified_rename_results.is_empty());
}

#[test]
fn test_unified_naming_zero_padding() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "ep1.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "Test".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "1".to_string();
    app.unified_pattern = "{title}.S{season}E{episode}".to_string();

    app.generate_unified_preview();

    // Should have zero-padded season and episode
    assert!(app.unified_preview[0].contains("S01E01"));
}

#[test]
fn test_unified_naming_double_digit_season_episode() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "ep1.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "Test".to_string();
    app.unified_season = "12".to_string();
    app.unified_start_episode = "15".to_string();
    app.unified_pattern = "{title}.S{season}E{episode}".to_string();

    app.generate_unified_preview();

    // Should have correct double digit formatting
    assert!(app.unified_preview[0].contains("S12E15"));
}

#[test]
fn test_unified_naming_pattern_placeholders() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "video.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];
    app.unified_show_name = "MyShow".to_string();
    app.unified_season = "1".to_string();
    app.unified_start_episode = "1".to_string();

    // Test with different pattern formats
    app.unified_pattern = "{title}_Season{season}_Episode{episode}".to_string();
    app.generate_unified_preview();
    assert!(app.unified_preview[0].contains("MyShow_Season01_Episode01"));
}

#[test]
fn test_unified_naming_state_transitions() {
    let mut app = App::new();
    app.files = vec![
        FileItem { name: "A.mkv".to_string(), is_dir: false, size: Some(1000) },
        FileItem { name: "B.mkv".to_string(), is_dir: false, size: Some(1000) },
    ];

    // Initial state
    assert!(!app.show_unified_input);
    assert_eq!(app.unified_show_name, "");

    // Start unified mode
    app.start_unified_mode();
    assert!(app.show_unified_input);
    assert_eq!(app.unified_focus, openlist_tui::app::UnifiedFocus::ShowName);

    // Input show name
    app.unified_show_name = "Test Show".to_string();
    app.generate_unified_preview();
    assert!(!app.unified_preview.is_empty());

    // Cancel
    app.cancel_unified();
    assert!(!app.show_unified_input);
    assert!(app.unified_show_name.is_empty());
}
