use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

use nusb::transfer::{ControlOut, ControlType, Recipient};
use nusb::{Device, MaybeFuture, list_devices};

use crate::Command;
use crate::control::Control;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);
const VID: u16 = 0x0416;
const PID: u16 = 0x9391;

/// A USB missile launcher.
#[derive(Debug)]
pub struct MissileLauncher {
    device: Device,
}

impl MissileLauncher {
    /// Crate a new USB missile launcher.
    #[must_use]
    pub const fn new(device: Device) -> Self {
        Self { device }
    }

    /// Open the missile launcher with the given VID and PID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if opening the launcher fails.
    pub fn open_with_vid_and_pid(vid: u16, pid: u16) -> Result<MissileLauncher> {
        Ok(Self::new(
            list_devices()
                .wait()?
                .find(|dev| dev.vendor_id() == vid && dev.product_id() == pid)
                .ok_or_else(|| Error::new(ErrorKind::NotFound, "device not found"))?
                .open()
                .wait()?,
        ))
    }

    /// Open the missile launcher with the default VID and PID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if opening the launcher fails.
    pub fn open() -> Result<MissileLauncher> {
        Self::open_with_vid_and_pid(VID, PID)
    }

    /// Send a command with a given timeout to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command fails.
    pub fn send_command_with_timeout(&mut self, command: Command, timeout: Duration) -> Result<()> {
        Ok(self
            .device
            .claim_interface(0)
            .wait()?
            .control_out(
                ControlOut {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Interface,
                    request: 0x09,
                    value: 0x0300,
                    index: 0x0000,
                    data: &command.into_payload(),
                },
                timeout,
            )
            .wait()?)
    }

    /// Send a command to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending the command fails.
    pub fn send_command(&mut self, command: Command) -> Result<()> {
        self.send_command_with_timeout(command, DEFAULT_TIMEOUT)
    }
}

impl Control for MissileLauncher {
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
