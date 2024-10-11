use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::layouts::Popup;

pub struct Component {
    message: String,
}

pub struct ComponentParameters {
    pub message: String,
}

impl Component {
    pub fn new(parameters: ComponentParameters) -> Self {
        let ComponentParameters { message } = parameters;

        Self { message }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let area = Popup::default().area(area);
        let [area] = Layout::default()
            .constraints([Constraint::Max(3)])
            .areas(area);

        let block = Block::default().borders(Borders::all());
        let paragraph = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }
}
