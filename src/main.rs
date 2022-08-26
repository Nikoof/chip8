pub mod state;
pub mod ui;
pub mod instruction;

use std::io;
use scopeguard::defer;
use anyhow::Result;
use tui::{
    backend::CrosstermBackend,
    style::Color,
    widgets::canvas::Points
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode}
};

use ui::ui;
use state::State;

fn main() -> Result<()> {
    let mut terminal = create_terminal(io::stdout())?;
    init_terminal()?;
    defer!{ deinit_terminal().unwrap(); }

    let mut state = State::default();
    state.load_program("./roms/IBM Logo.ch8")?;

    loop {
        state.update()?;
        let coords = state.get_points();
        let points = Points {
            coords: &coords,
            color: Color::White
        };
        terminal.draw(|f| ui(f, points))?;

        if let Ok(event_available) = event::poll(std::time::Duration::from_secs(0)) {
            if event_available {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
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
