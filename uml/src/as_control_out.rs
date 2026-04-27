use nusb::transfer::{ControlOut, ControlType, Recipient};

/// Extension trait to return a data payload as a `ControlOut` packet.
pub trait AsControlOut {
    /// Return the data payload as a `ControlOut` packet.
    fn as_control_out(&self) -> ControlOut<'_>;
}

impl AsControlOut for [u8] {
    fn as_control_out(&self) -> ControlOut<'_> {
        ControlOut {
            control_type: ControlType::Class,
            recipient: Recipient::Interface,
            request: 0x09,
            value: 0x0300,
            index: 0x0000,
            data: self,
        }
    }
}
