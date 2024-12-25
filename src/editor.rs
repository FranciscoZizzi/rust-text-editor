use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers};
use core::cmp::{ min, max };
mod terminal;
use terminal::{ Terminal, Position, Size };
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    cursor_position: Position,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false, cursor_position: Position { column: 1, row: 0 } }
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
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {code, modifiers, kind: KeyEventKind::Press, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_cursor(code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_cursor(&mut self, code: &KeyCode) -> Result<(), Error> {
        let Size{columns, rows} = Terminal::size()?;
        let Position{row, column} = self.cursor_position;
        match code {
            KeyCode::Left => {
                self.cursor_position = Position {row, column: max(column.saturating_sub(1), 1)};
            }
            KeyCode::Right => {
                self.cursor_position = Position {row, column: min(column.saturating_add(1), columns - 1)};
            }
            KeyCode::Up => {
                self.cursor_position = Position {row: row.saturating_sub(1), column};
            }
            KeyCode::Down => {
                self.cursor_position = Position {row: min(row.saturating_add(1), rows - 1), column};
            }
            KeyCode::PageUp => {
                self.cursor_position = Position {row: 0, column};
            }
            KeyCode::PageDown => {
                self.cursor_position = Position {row: 0, column};
            }
            KeyCode::Home => {
                self.cursor_position = Position {row: 0, column};
            }
            KeyCode::End => {
                self.cursor_position = Position {row: rows - 1, column};
            }
            _ => ()
        }
        Terminal::move_cursor_to(self.cursor_position)?;
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.cursor_position)?;
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
        Terminal::move_cursor_to(Position {row: 0, column: 0})?;
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

