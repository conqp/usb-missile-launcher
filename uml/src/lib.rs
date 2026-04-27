//! Library to control a USB missile launcher.

pub use nusb::Device;

pub use self::as_control_out::AsControlOut;
pub use self::command::Command;
pub use self::missile_launcher::MissileLauncher;

mod as_control_out;
mod command;
mod missile_launcher;
