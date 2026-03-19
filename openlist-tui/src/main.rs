mod api;
mod app;
mod config;
mod error;
mod models;
mod task;
mod ui;
mod validate;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use crate::app::App;
use crate::config::Config;

fn main() -> Result<()> {
    let config = Config::load()?;
    let mut app = App::with_config(config);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render::render(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // Handle login screen input
                if app.show_login_screen {
                    if app.is_logging_in {
                        // Wait for login to complete, only allow Esc to cancel
                        if key.code == KeyCode::Esc {
                            app.clear_login();
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('c')
                                if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                            {
                                return Ok(())
                            }
                            KeyCode::Esc => {
                                app.clear_login();
                            }
                            KeyCode::Enter => {
                                // Submit login
                                if app.username_input.is_empty() || app.password_input.is_empty() {
                                    app.error_message = Some("用户名和密码不能为空".to_string());
                                    app.show_error_popup = true;
                                    app.show_login_screen = false;
                                } else {
                                    // Perform login - set logging in state
                                    app.submit_login();
                                    // Note: Actual API call would be async
                                    // For now, just simulate success/failure
                                    // In production, use tokio runtime for async call
                                }
                            }
                            KeyCode::Backspace => {
                                match app.login_focus {
                                    crate::app::LoginFocus::Username => {
                                        app.delete_last_username_char();
                                    }
                                    crate::app::LoginFocus::Password => {
                                        app.delete_last_password_char();
                                    }
                                }
                            }
                            KeyCode::Tab => {
                                // Toggle between username and password input
                                app.toggle_login_focus();
                            }
                            KeyCode::Char(c) => {
                                // Text input for username/password based on focus
                                match app.login_focus {
                                    crate::app::LoginFocus::Username => {
                                        if app.username_input.len() < 50 {
                                            app.append_to_username(c);
                                        }
                                    }
                                    crate::app::LoginFocus::Password => {
                                        if app.password_input.len() < 50 {
                                            app.append_to_password(c);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    continue;
                }

                // Normal mode key bindings
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c')
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        return Ok(())
                    }
                    KeyCode::Char('l') => {
                        // Open login screen
                        app.start_login();
                    }
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                    KeyCode::Tab => app.toggle_focus(),
                    KeyCode::Enter => {
                        // TODO: 进入目录
                    }
                    _ => {}
                }
            }
        }
    }
}
