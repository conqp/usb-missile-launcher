/// Control a missile launcher.
pub trait Control {
    /// Aim the missile launcher left.
    fn left(&mut self) -> rusb::Result<()>;

    /// Aim the missile launcher right.
    fn right(&mut self) -> rusb::Result<()>;

    /// Aim the missile launcher up.
    fn up(&mut self) -> rusb::Result<()>;

    /// Aim the missile launcher down.
    fn down(&mut self) -> rusb::Result<()>;

    /// Fire the missile launcher.
    fn fire(&mut self) -> rusb::Result<()>;

    /// Stop the missile launcher.
    fn stop(&mut self) -> rusb::Result<()>;
}
