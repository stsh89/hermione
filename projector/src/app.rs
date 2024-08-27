use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::{Block, Paragraph},
};

pub struct App {}

impl App {
    pub fn run(&self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|frame| {
                frame.render_widget(
                    Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
                    frame.area(),
                );
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        ratatui::restore();

        Ok(())
    }
}
