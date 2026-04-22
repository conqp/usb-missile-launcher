use std::time::Duration;

use log::debug;
use rusb::{Context, DeviceHandle, UsbContext};

use crate::Command;
use crate::control::Control;

const VID: u16 = 0x0416;
const PID: u16 = 0x9391;
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

    /// Open the missile launcher from the given VID and PID.
    ///
    /// # Errors
    ///
    /// Returns an [`rusb::Error`] if opening the device fails.
    pub fn from_descriptor(vid: u16, pid: u16) -> rusb::Result<Self> {
        for device in Context::new()?.devices()?.iter() {
            match device.device_descriptor() {
                Ok(descriptor) => {
                    if descriptor.vendor_id() == vid && descriptor.product_id() == pid {
                        return device.open().map(Self::new);
                    }
                }
                Err(error) => {
                    debug!("{error}")
                }
            };
        }

        Err(rusb::Error::NoDevice)
    }

    /// Open the missile launcher from the default VID and PID.
    ///
    /// # Errors
    ///
    /// Returns an [`rusb::Error`] if opening the device fails.
    pub fn open() -> rusb::Result<Self> {
        Self::from_descriptor(VID, PID)
    }

    /// Send a command with a given timeout to the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`rusb::Error`] if sending the command fails.
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
    /// Returns an [`rusb::Error`] if sending the command fails.
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
