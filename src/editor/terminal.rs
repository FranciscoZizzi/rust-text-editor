use crossterm::terminal::{disable_raw_mode, enable_raw_mode, ClearType, Clear, size};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn move_cursor_to(column: u16, row: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(column, row))?;
        Ok(())
    }
}

