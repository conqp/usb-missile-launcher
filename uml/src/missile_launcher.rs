use std::time::Duration;

use rusb::{Context, DeviceHandle};

use crate::Command;
use crate::control::Control;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

/// A USB missile launcher.
#[derive(Debug)]
pub struct MissileLauncher {
    handle: DeviceHandle<Context>,
}

impl MissileLauncher {
    /// Crate a new USB missile launcher.
    #[must_use]
    pub const fn new(handle: DeviceHandle<Context>) -> Self {
        Self { handle }
    }

    /// Send a command with a given timeout to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command fails.
    pub fn send_command_with_timeout(
        &mut self,
        command: Command,
        timeout: Duration,
    ) -> rusb::Result<usize> {
        self.handle
            .write_control(0x21, 0x09, 0x0300, 0x0000, &command.into_payload(), timeout)
    }

    /// Send a command to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command fails.
    pub fn send_command(&mut self, command: Command) -> rusb::Result<()> {
        self.send_command_with_timeout(command, DEFAULT_TIMEOUT)
            .map(drop)
    }
}

impl Control for MissileLauncher {
    fn left(&mut self) -> rusb::Result<()> {
        self.send_command(Command::Left)
    }

    fn right(&mut self) -> rusb::Result<()> {
        self.send_command(Command::Right)
    }

    fn up(&mut self) -> rusb::Result<()> {
        self.send_command(Command::Up)
    }

    fn down(&mut self) -> rusb::Result<()> {
        self.send_command(Command::Down)
    }

    fn fire(&mut self) -> rusb::Result<()> {
        self.send_command(Command::Fire)
    }

    fn stop(&mut self) -> rusb::Result<()> {
        self.send_command(Command::Stop)
    }
}
