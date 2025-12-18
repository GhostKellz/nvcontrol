//! Terminal management for TUI

use crate::NvResult;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, IsTerminal, Stdout};
use std::panic;

/// Terminal wrapper that handles setup/teardown properly
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    /// Initialize terminal - enters alternate screen and raw mode immediately
    pub fn init() -> NvResult<Self> {
        // Check for real terminal first
        if !io::stdout().is_terminal() {
            return Err(crate::NvControlError::RuntimeError(
                "TUI requires a terminal. Use a terminal emulator like Ghostty, Kitty, or Alacritty.".into()
            ));
        }

        // Enter raw mode and alternate screen FIRST
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        // Set panic hook to restore terminal on crash
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            Self::restore_terminal().unwrap_or_else(|e| {
                eprintln!("Failed to restore terminal: {}", e);
            });
            original_hook(panic_info);
        }));

        Ok(Self { terminal })
    }

    /// Get mutable reference to terminal for drawing
    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    /// Restore terminal to normal state
    fn restore_terminal() -> NvResult<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Clean exit - restore terminal and show cursor
    pub fn exit(&mut self) -> NvResult<()> {
        Self::restore_terminal()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        Self::restore_terminal().unwrap_or_else(|e| {
            eprintln!("Failed to restore terminal on drop: {}", e);
        });
    }
}
