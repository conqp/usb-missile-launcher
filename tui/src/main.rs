//! TUI controller for the USB missile launcher.

use std::process::ExitCode;

use log::error;
use uml::MissileLauncher;

use self::app::App;

mod app;
mod extended_terminal;
mod table;

fn main() -> ExitCode {
    env_logger::init();

    let Ok(missile_launcher) = MissileLauncher::open().inspect_err(|error| error!("{error}"))
    else {
        return ExitCode::FAILURE;
    };

    if let Err(error) = ratatui::run(|terminal| App::new(missile_launcher).run(terminal)) {
        error!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
