use crate::MissileLauncher;
use rusb::{Context, Error, UsbContext};

const VID: u16 = 0x0416;
const PID: u16 = 0x9391;

/// Extension trait for [`Context`] to open a missile launcher.
pub trait OpenMissileLauncher {
    /// Open the missile launcher.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] in case opening the missile launcher fails.
    fn open_missile_launcher(self) -> rusb::Result<MissileLauncher>;
}

impl OpenMissileLauncher for Context {
    fn open_missile_launcher(self) -> rusb::Result<MissileLauncher> {
        OpenMissileLauncher::open_missile_launcher(&self)
    }
}

impl OpenMissileLauncher for &Context {
    fn open_missile_launcher(self) -> rusb::Result<MissileLauncher> {
        self.open_device_with_vid_pid(VID, PID)
            .ok_or(Error::NoDevice)
            .map(MissileLauncher::new)
    }
}
