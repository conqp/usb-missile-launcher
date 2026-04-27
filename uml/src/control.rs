use std::io::Result;

/// Control a missile launcher.
pub trait Control {
    /// Yaw left.
    fn left(&mut self) -> Result<()>;

    /// Yaw right.
    fn right(&mut self) -> Result<()>;

    /// Pitch up.
    fn up(&mut self) -> Result<()>;

    /// Pitch down.
    fn down(&mut self) -> Result<()>;

    /// Fire missiles.
    fn fire(&mut self) -> Result<()>;

    /// Stop any current action.
    fn stop(&mut self) -> Result<()>;
}
