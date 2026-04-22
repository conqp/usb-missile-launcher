/// Control a missile launcher.
pub trait Control {
    /// Yaw left.
    fn left(&mut self) -> rusb::Result<()>;

    /// Yaw right.
    fn right(&mut self) -> rusb::Result<()>;

    /// Pitch up.
    fn up(&mut self) -> rusb::Result<()>;

    /// Pitch down.
    fn down(&mut self) -> rusb::Result<()>;

    /// Fire missiles.
    fn fire(&mut self) -> rusb::Result<()>;

    /// Stop any current action.
    fn stop(&mut self) -> rusb::Result<()>;
}
