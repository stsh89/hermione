use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, StatefulWidget},
};

pub use ratatui::widgets::ListState as State;

pub struct Widget<'a, T> {
    pub title: &'a str,
    pub items: &'a [T],
}

impl<'a, T> StatefulWidget for Widget<'a, T>
where
    &'a T: Into<ListItem<'a>>,
{
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::all()).title(self.title);

        let items: Vec<ListItem> = self.items.iter().map(Into::into).collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().yellow())
            .highlight_symbol("➤ ");

        list.render(area, buf, state);
    }
}
