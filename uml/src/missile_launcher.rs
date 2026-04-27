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
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn send_command_with_timeout(&mut self, command: Command, timeout: Duration) -> Result<()>;

    /// Send a command to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn send_command(&mut self, command: Command) -> Result<()> {
        self.send_command_with_timeout(command, DEFAULT_TIMEOUT)
    }

    /// Yaw the missile launcher left.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn left(&mut self) -> Result<()> {
        self.send_command(Command::Left)
    }

    /// Yaw the missile launcher right.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn right(&mut self) -> Result<()> {
        self.send_command(Command::Right)
    }

    /// Pitch the missile launcher up.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn up(&mut self) -> Result<()> {
        self.send_command(Command::Up)
    }

    /// Pitch the missile launcher down.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn down(&mut self) -> Result<()> {
        self.send_command(Command::Down)
    }

    /// Fire the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
    fn fire(&mut self) -> Result<()> {
        self.send_command(Command::Fire)
    }

    /// Stop any previous action on the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the USB device fails.
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
