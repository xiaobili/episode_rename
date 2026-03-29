mod api;
mod app;
mod components;
mod config;
mod error;
mod input;
mod message;
mod models;
mod state;
mod task;
mod tasks;
mod update;
mod validate;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

use crate::app::App;
use crate::config::Config;
use crate::input::{key_to_message, should_quit, task_result_to_message};
use crate::message::{AsyncMsg, Message};
use crate::tasks::{handle_special_keys, process_pending_renames, start_auto_login};
use crate::update::update;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let mut app = App::with_config(config);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_app(&mut terminal, &mut app).await;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    result
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let refresh_interval = Duration::from_millis(50);
    let mut last_refresh = std::time::Instant::now();
    let mut need_reload = false;
    start_auto_login(app);

    loop {
        while let Ok(result) = app.async_state.task_channel.rx.try_recv() {
            let msg = task_result_to_message(result);
            let reload = matches!(
                &msg,
                Message::Async(AsyncMsg::LoginResult(Ok(_)))
                    | Message::Async(AsyncMsg::AutoLoginResult(Ok(_)))
                    | Message::Async(AsyncMsg::BatchRenameResult(Ok(())))
            );
            update(app, msg);
            if reload {
                need_reload = true;
            }
        }
        if need_reload && app.auth.is_authenticated {
            need_reload = false;
            if let Err(e) = app.load_directory_contents().await {
                app.handle_api_error(e);
            }
        }
        process_pending_renames(app).await;

        let now = std::time::Instant::now();
        let loading = app.async_state.pending_task.is_loading();
        if loading && now.duration_since(last_refresh) >= refresh_interval {
            app.async_state.pending_task.advance_spinner();
        }
        if !loading || now.duration_since(last_refresh) >= refresh_interval {
            terminal.draw(|f| components::render(f, app))?;
            last_refresh = now;
        }

        let timeout = if loading {
            refresh_interval
        } else {
            Duration::from_millis(100)
        };
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if should_quit(key) {
                    return Ok(());
                }
                match key_to_message(app, key) {
                    Some(msg) => update(app, msg),
                    None => handle_special_keys(terminal, app, key).await?,
                }
            }
        }
    }
}
