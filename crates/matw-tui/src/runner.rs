//! TUI main loop
//!
//! Main entry point for the terminal UI application.

use crate::{App, Event, EventHandler, UI};
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use matw_agent::Agent;
use matw_ai::providers::GLMProvider;
use matw_core::Session;
use matw_tools::all_tools;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Run the TUI application
pub async fn run() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup app
    let session = Session::new(std::env::current_dir()?);
    let tools_raw = all_tools();
    let tools: Vec<_> = tools_raw
        .into_iter()
        .map(|t| std::sync::Arc::from(t) as std::sync::Arc<dyn matw_tools::Tool>)
        .collect();

    // Create provider and agent
    let provider = GLMProvider::new("test-key".to_string(), None);
    let agent = Agent::new(provider, tools.clone());

    let mut app = App::new(session, tools).with_agent(agent);
    let mut events = EventHandler::new(250);

    // Main loop
    loop {
        terminal.draw(|f| UI::draw(f, &app))?;

        if let Some(event) = events.next().await {
            match event {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char(c) => app.handle_input(c),
                        KeyCode::Backspace => app.handle_backspace(),
                        KeyCode::Enter => app.submit_input().await,
                        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
                        _ => {}
                    }
                }
                Event::Tick => {
                    // Periodic updates (e.g., status changes)
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
