use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{self, Block, Borders, ListItem, ListState, StatefulWidget},
};

pub struct List<'a, T> {
    pub title: &'a str,
    pub items: &'a [T],
}

impl<'a, T> StatefulWidget for List<'a, T>
where
    &'a T: Into<ListItem<'a>>,
{
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::all()).title(self.title);

        let items: Vec<ListItem> = self.items.iter().map(Into::into).collect();
        let list = widgets::List::new(items)
            .block(block)
            .highlight_style(Style::default().yellow());

        list.render(area, buf, state);
    }
}
