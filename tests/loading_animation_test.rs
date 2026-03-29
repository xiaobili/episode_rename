use openlist_tui::app::App;
use openlist_tui::task::PendingTask;
use openlist_tui::update::*;

#[test]
fn test_loading_start() {
    let mut app = App::default();

    // Initially should not be loading
    assert!(!app.async_state.pending_task.is_loading());
    assert!(app.ui.loading_message.is_none());
    assert!(app.ui.loading_progress.is_none());

    // Start loading
    start_loading(&mut app, "Loading files...".to_string());

    // Should be in loading state
    assert!(app.ui.loading_message.is_some());
    assert_eq!(app.ui.loading_message, Some("Loading files...".to_string()));
    assert_eq!(app.ui.loading_spinner_frame, 0);
}

#[test]
fn test_loading_stop() {
    let mut app = App::default();

    // Start loading
    start_loading(&mut app, "Loading...".to_string());
    assert!(app.ui.loading_message.is_some());

    // Stop loading
    stop_loading(&mut app);

    // Should not be loading anymore
    assert!(app.ui.loading_message.is_none());
    assert!(app.ui.loading_progress.is_none());
    assert_eq!(app.ui.loading_spinner_frame, 0);
}

#[test]
fn test_progress_update() {
    let mut app = App::default();

    // Initially no progress
    assert!(app.ui.loading_progress.is_none());

    // Update progress
    update_progress(&mut app, 5, 10);

    // Should have progress
    assert!(app.ui.loading_progress.is_some());
    assert_eq!(app.ui.loading_progress, Some((5, 10)));

    // Update progress again
    update_progress(&mut app, 8, 10);
    assert_eq!(app.ui.loading_progress, Some((8, 10)));
}

#[test]
fn test_spinner_frame_rotation() {
    let mut app = App::default();

    // Start loading
    start_loading(&mut app, "Loading...".to_string());

    // Initial frame
    assert_eq!(app.ui.loading_spinner_frame, 0);

    // Advance spinner 10 times - should cycle back to 0
    for i in 1..=10 {
        advance_spinner(&mut app);
        if i < 10 {
            assert_eq!(app.ui.loading_spinner_frame, i);
        } else {
            // After 10 advances, should be back to 0
            assert_eq!(app.ui.loading_spinner_frame, 0);
        }
    }
}

#[test]
fn test_spinner_characters() {
    let mut app = App::default();

    // Start loading
    start_loading(&mut app, "Loading...".to_string());

    // Test all spinner characters cycle correctly
    let expected_chars: Vec<char> = vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

    for expected_char in expected_chars.iter() {
        assert_eq!(get_spinner_char(&app), *expected_char);
        advance_spinner(&mut app);
    }

    // After full cycle, should start from beginning
    assert_eq!(get_spinner_char(&app), '⠋');
}

#[test]
fn test_pending_task_loading_state() {
    // Test Idle state
    let idle = PendingTask::Idle;
    assert!(!idle.is_loading());
    assert!(idle.get_progress().is_none());
    assert!(idle.get_message().is_none());

    // Test Loading state
    let loading = PendingTask::Loading {
        id: 1,
        message: "Loading...".to_string(),
        spinner_frame: 0,
    };
    assert!(loading.is_loading());
    assert!(loading.get_progress().is_none());
    assert_eq!(loading.get_message(), Some("Loading..."));

    // Test Renaming state
    let renaming = PendingTask::Renaming {
        id: 2,
        total: 10,
        completed: 5,
        message: "Renaming files...".to_string(),
        spinner_frame: 0,
    };
    assert!(renaming.is_loading());
    assert_eq!(renaming.get_progress(), Some((5, 10)));
    assert_eq!(renaming.get_message(), Some("Renaming files..."));
}

#[test]
fn test_pending_task_spinner() {
    let mut loading = PendingTask::Loading {
        id: 1,
        message: "Loading...".to_string(),
        spinner_frame: 0,
    };

    // Test initial spinner char
    assert_eq!(loading.get_spinner_char(), '⠋');

    // Advance and check spinner changes
    loading.advance_spinner();
    assert_eq!(loading.get_spinner_char(), '⠙');

    // Test Renaming state spinner
    let mut renaming = PendingTask::Renaming {
        id: 2,
        total: 10,
        completed: 0,
        message: "Renaming...".to_string(),
        spinner_frame: 5,
    };

    // Should start at frame 5
    let expected_chars: Vec<char> = vec!['⠴', '⠦', '⠧', '⠇', '⠏', '⠋'];
    for expected in expected_chars.iter() {
        assert_eq!(renaming.get_spinner_char(), *expected);
        renaming.advance_spinner();
    }
}

#[test]
fn test_app_pending_task_integration() {
    let mut app = App::default();

    // Set pending task to Loading
    app.async_state.pending_task = PendingTask::Loading {
        id: 1,
        message: "Connecting...".to_string(),
        spinner_frame: 0,
    };

    assert!(app.async_state.pending_task.is_loading());
    assert_eq!(
        app.async_state.pending_task.get_message(),
        Some("Connecting...")
    );

    // Advance spinner through pending task
    app.async_state.pending_task.advance_spinner();
    assert_eq!(app.async_state.pending_task.get_spinner_char(), '⠙');

    // Set pending task to Renaming with progress
    app.async_state.pending_task = PendingTask::Renaming {
        id: 2,
        total: 20,
        completed: 10,
        message: "Renaming...".to_string(),
        spinner_frame: 0,
    };

    assert_eq!(app.async_state.pending_task.get_progress(), Some((10, 20)));
    assert_eq!(
        app.async_state.pending_task.get_message(),
        Some("Renaming...")
    );
}

#[test]
fn test_loading_with_progress_percentage() {
    let mut app = App::default();

    start_loading(&mut app, "Batch processing...".to_string());

    // Test various progress percentages
    let test_cases: Vec<(usize, usize, f64)> = vec![
        (0, 10, 0.0),
        (5, 10, 50.0),
        (10, 10, 100.0),
        (25, 100, 25.0),
        (75, 100, 75.0),
    ];

    for (completed, total, _expected_percent) in test_cases {
        update_progress(&mut app, completed, total);
        assert_eq!(app.ui.loading_progress, Some((completed, total)));
    }
}

#[test]
fn test_spinner_frame_bounds() {
    let mut app = App::default();

    // Advance spinner many times - should never go out of bounds
    for _ in 0..100 {
        advance_spinner(&mut app);
        assert!(app.ui.loading_spinner_frame < 10);
    }

    // get_spinner_char should never panic
    for _ in 0..100 {
        let _char = get_spinner_char(&app);
        advance_spinner(&mut app);
    }
}

#[test]
fn test_loading_state_transitions() {
    let mut app = App::default();

    // Start loading
    start_loading(&mut app, "Step 1".to_string());
    assert!(app.ui.loading_message.is_some());
    assert_eq!(app.ui.loading_message, Some("Step 1".to_string()));

    // Update to different message (simulating new step)
    stop_loading(&mut app);
    start_loading(&mut app, "Step 2".to_string());
    assert_eq!(app.ui.loading_message, Some("Step 2".to_string()));
    assert_eq!(app.ui.loading_spinner_frame, 0); // Should reset

    // Add progress
    update_progress(&mut app, 3, 5);
    assert_eq!(app.ui.loading_progress, Some((3, 5)));

    // Complete
    stop_loading(&mut app);
    assert!(app.ui.loading_message.is_none());
    assert!(app.ui.loading_progress.is_none());
}
