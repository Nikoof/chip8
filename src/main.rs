pub mod ui;

use std::io;
use scopeguard::defer;
use anyhow::Result;
use tui::{
    backend::CrosstermBackend,
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode}
};

use ui::ui;

fn main() -> Result<()> {
    init_terminal()?;
    defer! {
        deinit_terminal().unwrap();
    }
    let mut terminal = create_terminal(io::stdout())?;

    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => continue
            }
        }
    }

    Ok(())
}

fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn create_terminal<W: io::Write>(buf: W) -> io::Result<tui::Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

fn deinit_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
