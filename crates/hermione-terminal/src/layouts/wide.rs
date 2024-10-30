use ratatui::layout::{Constraint, Direction, Rect};

pub struct WideLayout {
    layout: ratatui::layout::Layout,
}

impl WideLayout {
    pub fn new() -> Self {
        let layout = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Max(1)]);

        Self { layout }
    }

    pub fn areas(&self, area: Rect) -> [Rect; 2] {
        self.layout.areas(area)
    }
}
