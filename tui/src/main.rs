//! TUI controller for the USB missile launcher.

use std::error::Error;
use uml::MissileLauncher;

use self::app::App;

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    #[cfg(not(target_os = "windows"))]
    enhanced_tty::setup()?;

    let missile_launcher = MissileLauncher::open()?;
    ratatui::run(|terminal| App::new(missile_launcher).run(terminal))?;
    Ok(())
}


#[cfg(not(target_os = "windows"))]
mod enhanced_tty {
    use std::io::stdout;

    use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
    use crossterm::execute;
    use crossterm::terminal::enable_raw_mode;

    pub fn setup() -> std::io::Result<()> {
        enable_raw_mode()?;

        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )
    }
}