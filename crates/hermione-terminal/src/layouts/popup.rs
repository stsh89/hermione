use ratatui::layout::{Constraint, Flex, Layout, Rect};

const DEFAULT_PERCENT_X: u16 = 50;
const DEFAULT_PERCENT_Y: u16 = 30;

pub struct Popup {
    vertical: Layout,
    horizontal: Layout,
}

impl Popup {
    pub fn new(percent_x: u16, percent_y: u16) -> Self {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);

        Self {
            vertical,
            horizontal,
        }
    }

    pub fn area(&self, area: Rect) -> Rect {
        let [area] = self.vertical.areas(area);
        let [area] = self.horizontal.areas(area);
        area
    }
}

impl Default for Popup {
    fn default() -> Self {
        Self::new(DEFAULT_PERCENT_X, DEFAULT_PERCENT_Y)
    }
}
