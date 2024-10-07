use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
        let area = popup_area(area, 30, 10);

        let block = Block::default().borders(Borders::all());
        let paragraph = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
