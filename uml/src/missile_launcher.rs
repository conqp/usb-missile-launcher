use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

use nusb::{Device, MaybeFuture, list_devices};

use crate::{AsControlOut, Command};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);
const VID: u16 = 0x0416;
const PID: u16 = 0x9391;

/// A USB missile launcher.
pub trait MissileLauncher {
    /// Open the missile launcher with the given VID and PID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if opening the launcher fails.
    fn open_with_vid_and_pid(vid: u16, pid: u16) -> Result<Self>
    where
        Self: Sized;

    /// Open the missile launcher with the default VID and PID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if opening the launcher fails.
    fn open() -> Result<Self>
    where
        Self: Sized,
    {
        Self::open_with_vid_and_pid(VID, PID)
    }

    /// Send a command with a given timeout to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command fails.
    fn send_command_with_timeout(&mut self, command: Command, timeout: Duration) -> Result<()>;

    /// Send a command to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command fails.
    fn send_command(&mut self, command: Command) -> Result<()> {
        self.send_command_with_timeout(command, DEFAULT_TIMEOUT)
    }

    fn left(&mut self) -> Result<()> {
        self.send_command(Command::Left)
    }

    fn right(&mut self) -> Result<()> {
        self.send_command(Command::Right)
    }

    fn up(&mut self) -> Result<()> {
        self.send_command(Command::Up)
    }

    fn down(&mut self) -> Result<()> {
        self.send_command(Command::Down)
    }

    fn fire(&mut self) -> Result<()> {
        self.send_command(Command::Fire)
    }

    fn stop(&mut self) -> Result<()> {
        self.send_command(Command::Stop)
    }
}

impl MissileLauncher for Device {
    fn open_with_vid_and_pid(vid: u16, pid: u16) -> Result<Self> {
        Ok(list_devices()
            .wait()?
            .find(|dev| dev.vendor_id() == vid && dev.product_id() == pid)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "device not found"))?
            .open()
            .wait()?)
    }

    fn send_command_with_timeout(&mut self, command: Command, timeout: Duration) -> Result<()> {
        Ok(self
            .claim_interface(0)
            .wait()?
            .control_out(command.into_payload().as_control_out(), timeout)
            .wait()?)
    }
}
