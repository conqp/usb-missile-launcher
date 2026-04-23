//! Server to control a rocket launcher via a web proxy.

use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use log::{error, warn};
use remotectl_common::{Args, Command};
use reqwest::{Url, get};
use rusb::Context;
use tokio::time::sleep;
use uml::{Control, OpenMissileLauncher};

const TICK: Duration = Duration::from_millis(100);
const FAILURE_TICK: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();

    let Ok(mut missile_launcher) = Context::new()
        .and_then(OpenMissileLauncher::open_missile_launcher)
        .inspect_err(|error| error!("{error}"))
    else {
        return ExitCode::FAILURE;
    };

    let args = Args::parse();
    let Ok(url) = args.url().inspect_err(|error| error!("{error}")) else {
        return ExitCode::FAILURE;
    };

    loop {
        let Ok(command) = fetch_command(&url)
            .await
            .inspect_err(|error| warn!("{error}"))
        else {
            missile_launcher
                .stop()
                .unwrap_or_else(|error| error!("{error}"));
            sleep(FAILURE_TICK).await;
            continue;
        };

        match command {
            Command::Stop => missile_launcher.stop(),
            Command::Left => missile_launcher.left(),
            Command::Right => missile_launcher.right(),
            Command::Up => missile_launcher.up(),
            Command::Down => missile_launcher.down(),
            Command::Fire => missile_launcher.fire(),
        }
        .unwrap_or_else(|error| error!("{error}"));

        sleep(TICK).await;
    }
}

async fn fetch_command(url: &Url) -> reqwest::Result<Command> {
    get(url.clone()).await?.error_for_status()?.json().await
}
