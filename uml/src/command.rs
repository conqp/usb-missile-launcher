#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Command {
    Left = 0x08,
    Right = 0x04,
    Up = 0x02,
    Down = 0x01,
    Fire = 0x10,
    Stop = 0x00,
}

impl Command {
    /// Return the USB command payload.
    #[must_use]
    pub const fn into_payload(self) -> [u8; 5] {
        [0x5f, self as u8, 0xe0, 0xff, 0xfe]
    }
}
