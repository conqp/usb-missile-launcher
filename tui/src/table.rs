use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Style, Text, Widget};
use ratatui::widgets::Paragraph;

/// A table.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Table<const COLS: usize, const ROWS: usize, T> {
    elements: [[T; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize, T> From<[[T; COLS]; ROWS]> for Table<COLS, ROWS, T> {
    fn from(elements: [[T; COLS]; ROWS]) -> Self {
        Self { elements }
    }
}

impl<const COLS: usize, const ROWS: usize, T> Widget for Table<COLS, ROWS, T>
where
    T: Into<Text<'static>>,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Percentage(
                    u16::try_from(100usize / ROWS).expect("Percentage always fits."),
                ); ROWS],
            )
            .split(area);

        for (row_area, row) in rows.iter().zip(self.elements) {
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(
                        u16::try_from(100usize / COLS).expect("Percentage always fits."),
                    ); COLS],
                )
                .split(*row_area);

            for (cell_area, cell) in cols.iter().zip(row) {
                let cell = Paragraph::new(cell)
                    .style(Style::default())
                    .alignment(Alignment::Center);

                cell.render(*cell_area, buf);
            }
        }
    }
}
