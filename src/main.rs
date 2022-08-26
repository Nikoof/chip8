pub mod state;
pub mod ui;
pub mod instruction;

use std::{
    io,
    time::{Duration, Instant}
};
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
    state.load_program("./roms/programs/IBM Logo.ch8")?;

    let cpu_cycle_duration = Duration::from_nanos(1_000_000_000 / 700);
    let timer_cycle_duration = Duration::from_nanos(1_000_000_000 / 60);

    'emu: loop {
        let now = Instant::now();
        let mut elapsed = Duration::from_secs(0);

        state.update_timers();

        'cpu: loop {
            state.update_keys()?;
            state.tick()?;

            terminal.draw(|rect| ui(rect, Points {
                coords: &state.get_points(),
                color: Color::White
            }))?;

            if event::poll(Duration::from_secs(0))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Esc = key.code {
                        break 'emu;
                    }
                }
            }
            
            // Calculate how much of the duration (1M / 700 seconds) has passed while doing actual work.
            let time_passed = now.elapsed() - elapsed;
            elapsed += time_passed;

            if time_passed < cpu_cycle_duration {
                // Sleep off the rest of the duration.
                let time_left = cpu_cycle_duration - time_passed;
                elapsed += time_left;
                std::thread::sleep(time_left);
            }

            // Exit the cpu loop when 1/60 seconds have passed.
            if elapsed >= timer_cycle_duration {
                break 'cpu;
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
