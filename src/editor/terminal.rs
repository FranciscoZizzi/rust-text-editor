use crossterm::terminal::{ disable_raw_mode, enable_raw_mode, ClearType, Clear, size };
use crossterm::cursor::{ MoveTo, Hide, Show };
use crossterm::{ queue, Command };
use crossterm::style::Print;
use std::io::{ stdout, Write, Error };

#[derive(Clone, Copy)]
pub struct Size {
    pub columns: u16,
    pub rows: u16
}

#[derive(Clone, Copy)]
pub struct Position {
    pub column: u16,
    pub row: u16
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (columns, rows) = size()?;
        Ok(Size{columns, rows})
    }

    pub fn move_cursor_to(Position { column, row }: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(column, row))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}

