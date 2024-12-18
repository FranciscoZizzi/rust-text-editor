use crossterm::terminal::{ disable_raw_mode, enable_raw_mode, ClearType, Clear, size };
use crossterm::cursor::{ MoveTo, Hide, Show };
use crossterm::queue;
use crossterm::style::Print;
use std::io::{ stdout, Write, Error };

pub struct Size {
    pub columns: u16,
    pub rows: u16
}

pub struct Position {
    pub column: u16,
    pub row: u16
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { column: 0, row: 0 })?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (columns, rows) = size()?;
        Ok(Size{columns, rows})
    }

    pub fn move_cursor_to(Position { column, row }: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(column, row))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}

