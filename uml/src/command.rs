/// Commands available for the missile launcher.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Command {
    /// Stop any previous action.
    Stop = 0x00,

    /// Pitch down.
    Down = 0x01,

    /// Pitch up.
    Up = 0x02,

    /// Yaw right.
    Right = 0x04,

    /// Yaw left.
    Left = 0x08,

    /// Fire missiles.
    Fire = 0x10,
}

impl Command {
    /// Return the USB command payload.
    #[must_use]
    pub const fn into_payload(self) -> [u8; 5] {
        [0x5f, self as u8, 0xe0, 0xff, 0xfe]
    }
}
