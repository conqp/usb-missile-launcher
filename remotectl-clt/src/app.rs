use std::io;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use log::{debug, error, warn};
use ratatui::layout::Flex;
use ratatui::prelude::{Widget, *};
use ratatui::widgets::{Block, Borders};
use ratatui::{DefaultTerminal, Frame};
use remotectl_common::Command;
use reqwest::Url;
use reqwest::blocking::Client;

use crate::extended_terminal::ExtendedTerminal;
use crate::table::Table;

#[derive(Debug)]
pub struct App {
    client: Client,
    url: Url,
    last_command: Option<Command>,
    exit: bool,
}

impl App {
    /// Crate a new application.
    #[must_use]
    pub fn new(url: Url) -> Self {
        Self {
            client: Client::new(),
            url,
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
        terminal
            .setup_extended_events()
            .unwrap_or_else(|error| warn!("{error}"));

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

    fn exit(&mut self) {
        self.exit = true;
        self.execute_command(Command::Stop);
    }

    fn execute_command(&mut self, command: Command) {
        if let Some(last_command) = self.last_command.take()
            && command == last_command
        {
            self.client
                .post(self.url.clone())
                .json(&Command::Stop)
                .send()
                .map_or_else(|error| error!("{error}"), drop);
        }

        self.last_command.replace(command);
        self.client
            .post(self.url.clone())
            .json(&command)
            .send()
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
                KeyCode::Enter => self.execute_command(Command::Fire),
                other => debug!("Unsupported key pressed: {other:?}"),
            },
            KeyEventKind::Release => match key_event.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down | KeyCode::Enter => {
                    self.last_command.take();
                    self.execute_command(Command::Stop);
                }
                other => debug!("Unsupported key released: {other:?}"),
            },
            KeyEventKind::Repeat => debug!("Unsupported key repeat: {key_event:?}"),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Missile Launcher Control ".bold());

        // Center the block itself
        let centered = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .flex(Flex::Center)
            .split(area)[1];

        let outer_block = Block::default()
            .title(title.centered())
            .borders(Borders::ALL);

        // IMPORTANT: inner area excludes borders
        let inner = outer_block.inner(centered);

        outer_block.render(centered, buf);

        Table::from([["", "^", ""], ["<", "<Enter>", ">"], ["", "v", ""]]).render(inner, buf);
    }
}
