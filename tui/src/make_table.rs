use std::num::TryFromIntError;

use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Style, Text};
use ratatui::widgets::{Paragraph, Widget};

/// Create a table within a rectangle.
pub trait MakeTable {
    /// The error type that can occur.
    type Error;

    /// Create a table from the given elements and render it in the given buffer.
    fn make_table<'a, const COLS: usize, const ROWS: usize, T>(
        self,
        elements: [[T; COLS]; ROWS],
        buffer: &mut Buffer,
    ) -> Result<(), Self::Error>
    where
        T: Into<Text<'a>>;
}

impl MakeTable for Rect {
    type Error = TryFromIntError;

    fn make_table<'a, const COLS: usize, const ROWS: usize, T>(
        self,
        elements: [[T; COLS]; ROWS],
        buffer: &mut Buffer,
    ) -> Result<(), Self::Error>
    where
        T: Into<Text<'a>>,
    {
        // Build 3 rows inside inner block
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(u16::try_from(100usize / ROWS)?); ROWS])
            .split(self);

        for (row_area, row) in rows.iter().zip(elements) {
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(u16::try_from(100usize / COLS)?); COLS])
                .split(*row_area);

            for (cell_area, cell) in cols.iter().zip(row) {
                let cell = Paragraph::new(cell)
                    .style(Style::default())
                    .alignment(Alignment::Center);

                cell.render(*cell_area, buffer);
            }
        }

        Ok(())
    }
}
