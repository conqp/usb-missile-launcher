use std::io::Result;

use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
use ratatui::DefaultTerminal;

/// Set up extended terminal functionality.
pub trait ExtendedTerminal {
    /// Set up the terminal for extended event handline.
    fn setup_extended_events(&mut self) -> Result<()>;
}

impl ExtendedTerminal for DefaultTerminal {
    fn setup_extended_events(&mut self) -> Result<()> {
        enable_raw_mode()?;

        execute!(
            self.backend_mut(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )
    }
}
