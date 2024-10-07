use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Scrollbar, ScrollbarOrientation, StatefulWidget},
};

pub use ratatui::widgets::ScrollbarState as State;

pub struct Widget;

impl StatefulWidget for Widget {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        scroll.render(area, buf, state);
    }
}
