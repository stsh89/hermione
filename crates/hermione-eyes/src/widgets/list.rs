use crate::colors::Color;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, List, ListItem, StatefulWidget},
};

pub use ratatui::widgets::ListState as State;

pub struct Widget<'a, T> {
    items: &'a [T],
    block: Option<Block<'a>>,
}

impl<'a, T> StatefulWidget for Widget<'a, T>
where
    &'a T: Into<ListItem<'a>>,
{
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<ListItem> = self.items.iter().map(Into::into).collect();
        let mut list = List::new(items)
            .highlight_style(*Color::highlight())
            .highlight_symbol("➤ ");

        if let Some(block) = self.block {
            list = list.block(block);
        }

        list.render(area, buf, state);
    }
}

impl<'a, T> Widget<'a, T> {
    pub fn block(self, block: Block<'a>) -> Self {
        Self {
            block: Some(block),
            ..self
        }
    }

    pub fn new(items: &'a [T]) -> Self {
        Self { items, block: None }
    }
}
