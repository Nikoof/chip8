use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    widgets::canvas::{Canvas, Points, Painter},
    style::Color
};

pub fn ui<B: Backend>(f: &mut tui::Frame<B>) {
    let root_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ].as_ref())
        .split(f.size());
    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15)
        ].as_ref())
        .split(root_layout[0]);
    let memview = Block::default()
        .title("Memory View")
        .borders(Borders::ALL);
    let display = Block::default()
        .title("Display")
        .borders(Borders::ALL);
    let console = Block::default()
        .title("Console")
        .borders(Borders::ALL);
    let log = Block::default()
        .title("Log")
        .borders(Borders::ALL);
    
    let canvas = Canvas::default()
        .block(display)
        .x_bounds([0.0, 64.0])
        .y_bounds([0.0, 32.0])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: [(0.0, 0.0), (64.0, 32.0)].as_ref(),
                color: Color::White
            });
        });

    f.render_widget(memview, top_layout[0]);
    f.render_widget(canvas, top_layout[1]);
    f.render_widget(console, top_layout[2]);
    f.render_widget(log, root_layout[1]);
}

