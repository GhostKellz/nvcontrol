//! Event handling for TUI

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Events that can occur in the TUI
#[derive(Clone, Debug)]
pub enum Event {
    /// Regular tick for updates
    Tick,
    /// Keyboard input
    Key(KeyEvent),
    /// Mouse input
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
}

/// Threaded event handler - polls for input without blocking main loop
pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
    _tx: mpsc::Sender<Event>,
    _handle: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Create new event handler with specified tick rate in milliseconds
    pub fn new(tick_rate_ms: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate_ms);
        let (tx, rx) = mpsc::channel();

        let handler_tx = tx.clone();
        let handle = thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                // Calculate timeout until next tick
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::ZERO);

                // Poll for events with timeout
                if event::poll(timeout).unwrap_or(false) {
                    if let Ok(event) = event::read() {
                        let result = match event {
                            CrosstermEvent::Key(key) => handler_tx.send(Event::Key(key)),
                            CrosstermEvent::Mouse(mouse) => handler_tx.send(Event::Mouse(mouse)),
                            CrosstermEvent::Resize(w, h) => handler_tx.send(Event::Resize(w, h)),
                            _ => Ok(()),
                        };

                        // Exit thread if receiver is gone
                        if result.is_err() {
                            break;
                        }
                    }
                }

                // Send tick event
                if last_tick.elapsed() >= tick_rate {
                    if handler_tx.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self {
            rx,
            _tx: tx,
            _handle: handle,
        }
    }

    /// Get next event (blocking)
    pub fn next(&self) -> Option<Event> {
        self.rx.recv().ok()
    }

    /// Try to get next event (non-blocking)
    pub fn try_next(&self) -> Option<Event> {
        self.rx.try_recv().ok()
    }
}
