//! Event handling for terminal events
//!
//! Provides async event loop with key, mouse, resize, and tick events.

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use std::time::Duration;
use tokio::sync::mpsc as tokio_mpsc;

/// Terminal events
pub enum Event {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize event
    Resize(u16, u16),
    /// Tick event (periodic)
    Tick,
}

/// Async event handler
pub struct EventHandler {
    sender: tokio_mpsc::UnboundedSender<Event>,
    receiver: tokio_mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    /// Create a new event handler with the specified tick rate
    pub fn new(tick_rate_ms: u64) -> Self {
        let (sender, receiver) = tokio_mpsc::unbounded_channel();

        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let mut last_tick = std::time::Instant::now();
            let tick_duration = Duration::from_millis(tick_rate_ms);

            loop {
                let timeout = tick_duration
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if timeout.is_zero() || event::poll(timeout).unwrap_or(false) {
                    if let Ok(event) = event::read() {
                        match event {
                            CrosstermEvent::Key(key) => {
                                if key.kind == KeyEventKind::Press {
                                    sender_clone.send(Event::Key(key)).ok();
                                }
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                sender_clone.send(Event::Mouse(mouse)).ok();
                            }
                            CrosstermEvent::Resize(x, y) => {
                                sender_clone.send(Event::Resize(x, y)).ok();
                            }
                            _ => {}
                        }
                    }
                }

                if last_tick.elapsed() >= tick_duration {
                    sender_clone.send(Event::Tick).ok();
                    last_tick = std::time::Instant::now();
                }
            }
        });

        Self { sender, receiver }
    }

    /// Get the next event
    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_handler_creation() {
        let handler = EventHandler::new(250);
        // Handler is created successfully
        drop(handler);
    }
}
