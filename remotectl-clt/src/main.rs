//! TUI controller for the USB missile launcher.

use std::process::ExitCode;

use clap::Parser;
use log::error;
use remotectl_common::Args;

use self::app::App;

mod app;
mod extended_terminal;
mod table;

fn main() -> ExitCode {
    env_logger::init();

    let args = Args::parse();

    let Ok(url) = args.url().inspect_err(|error| error!("{error}")) else {
        return ExitCode::FAILURE;
    };

    if let Err(error) = ratatui::run(|terminal| App::new(url).run(terminal)) {
        error!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
