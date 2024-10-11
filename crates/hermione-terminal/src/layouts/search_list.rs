use ratatui::layout::{Constraint, Direction, Rect};

pub struct Layout {
    layout: ratatui::layout::Layout,
}

impl Layout {
    pub fn new() -> Self {
        let layout = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(1)]);

        Self { layout }
    }

    pub fn areas(&self, area: Rect) -> [Rect; 2] {
        self.layout.areas(area)
    }
}
