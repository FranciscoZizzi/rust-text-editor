use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use terminal::{ Terminal, Position, Size };
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {code, modifiers, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position {column: 0, row: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn draw_welcome_message() -> Result<(), Error> {
        let Size {columns, rows} = Terminal::size()?;
        Terminal::move_cursor_to(Position {column: columns / 2 - (NAME.len() / 2) as u16, row: rows / 3})?;
        Terminal::print(NAME)?;
        Terminal::move_cursor_to(Position {column: columns / 2 - (VERSION.len() / 2) as u16, row: rows / 3 + 1})?;
        Terminal::print(VERSION)?;
        Ok(())
    }

    pub fn draw_rows() -> Result<(), Error> {
        let rows = Terminal::size()?.rows;
        for i in 0..rows {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if i + 1 < rows {
                Terminal::print("\r\n")?;
            }
        }
        Self::draw_welcome_message()?;
        Ok(())
    }
}

