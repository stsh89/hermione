use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub struct Popup {
    area: Rect,
}

impl Popup {
    fn area(&self, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);

        let [area] = vertical.areas(self.area);
        let [area] = horizontal.areas(area);

        area
    }

    pub fn new(area: Rect) -> Self {
        Self { area }
    }

    pub fn wide_area(&self) -> Rect {
        self.area(50, 50)
    }
}
