pub(crate) mod app;
mod events;
pub mod views;

use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{AppState, AppView};

/// Set up the terminal, run the TUI event loop, and restore the terminal on exit.
pub fn run(vault_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new(vault_path);
    let result = run_loop(&mut terminal, &mut app);

    // Always restore the terminal, even on error.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| render(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                events::handle_key(app, key);
            }
        }

        // Clipboard auto-clear: once the deadline passes, overwrite with empty string.
        if let Some(deadline) = app.clipboard_clear_at {
            if Instant::now() >= deadline {
                app.clipboard_clear_at = None;
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    let _ = cb.set_text("");
                }
                app.status = Some("Clipboard cleared.".to_string());
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn render(f: &mut ratatui::Frame, app: &AppState) {
    match &app.view {
        AppView::Locked { .. } => views::unlock::render(f, app),
        AppView::List { .. } => views::list::render(f, app),
        AppView::Detail { .. } => views::detail::render(f, app),
        AppView::Form { .. } => views::form::render(f, app),
        AppView::Help => views::help::render(f, app),
    }
}
