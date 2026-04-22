use std::io;
use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use log::{debug, error};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use uml::{Command, Control, MissileLauncher};

#[derive(Debug)]
pub struct App {
    missile_launcher: MissileLauncher,
    last_command: Option<Command>,
    exit: bool,
}

impl App {
    /// Crate a new application.
    #[must_use]
    pub const fn new(missile_launcher: MissileLauncher) -> Self {
        Self {
            missile_launcher,
            last_command: None,
            exit: false,
        }
    }

    /// Run the application's main loop until the user quits.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if any I/O error occurs.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame<'_>) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            self.handle_key_event(key_event);
        }

        Ok(())
    }

    const fn exit(&mut self) {
        self.exit = true;
    }

    fn execute_command(&mut self, command: Command) {
        if let Some(last_command) = self.last_command.take()
            && command == last_command
        {
            return self
                .missile_launcher
                .stop()
                .unwrap_or_else(|error| error!("{error}"));
        }

        self.last_command.replace(command);
        self.missile_launcher
            .send_command(command, Duration::from_secs(1))
            .map_or_else(|error| error!("{error}"), drop);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.kind {
            KeyEventKind::Press => match key_event.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Left => self.execute_command(Command::Left),
                KeyCode::Right => self.execute_command(Command::Right),
                KeyCode::Up => self.execute_command(Command::Up),
                KeyCode::Down => self.execute_command(Command::Down),
                other => debug!("Unsupported key pressed: {other:?}"),
            },
            KeyEventKind::Release => match key_event.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down | KeyCode::Enter => {
                    self.last_command.take();
                    self.missile_launcher
                        .stop()
                        .unwrap_or_else(|error| error!("{error}"));
                }
                other => debug!("Unsupported key released: {other:?}"),
            },
            KeyEventKind::Repeat => debug!("Unsupported key repeat: {key_event:?}"),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Paragraph::new("Foobar")
            .centered()
            .block(block)
            .render(area, buf);
    }
}
