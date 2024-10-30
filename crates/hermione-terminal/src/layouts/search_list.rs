use ratatui::layout::{Constraint, Direction, Rect};

pub struct SearchListLayout {
    layout: ratatui::layout::Layout,
}

impl SearchListLayout {
    pub fn new() -> Self {
        let layout = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(3), Constraint::Max(3)]);

        Self { layout }
    }

    pub fn areas(&self, area: Rect) -> [Rect; 2] {
        self.layout.areas(area)
    }
}
