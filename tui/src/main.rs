//! TUI controller for the USB missile launcher.

use std::error::Error;

use uml::MissileLauncher;

use self::app::App;

mod app;
mod extended_terminal;
mod make_table;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let missile_launcher = MissileLauncher::open()?;
    ratatui::run(|terminal| App::new(missile_launcher).run(terminal))?;
    Ok(())
}
