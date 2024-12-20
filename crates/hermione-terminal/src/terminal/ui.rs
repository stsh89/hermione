use crate::program::{Context, State};
use ratatui::{
    layout::{Constraint, Direction},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListState, Paragraph, Widget},
    Frame,
};

pub fn render(frame: &mut Frame, state: &State) {
    let [header, content, footer] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(1),
            Constraint::Min(1),
            Constraint::Max(1),
        ])
        .areas(frame.area());

    let [list_area, search_area] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3), Constraint::Max(3)])
        .areas(content);

    frame.render_widget(title(state), header);

    let mut help_line = Text::default();

    help_line.push_span("Press ");
    help_line.push_span(Span::from("q "));
    help_line.push_span("to quit");

    let block = Block::default().borders(Borders::ALL);
    let list = List::new(
        state
            .list
            .items
            .iter()
            .map(|item| item.text.clone())
            .collect::<Vec<_>>(),
    )
    .block(block)
    .highlight_symbol(">");
    let mut list_state = ListState::default();

    if !state.list.items.is_empty() {
        list_state.select(Some(state.list.cursor));
    }

    frame.render_stateful_widget(list, list_area, &mut list_state);

    let block = Block::default().borders(Borders::ALL);
    let search = Paragraph::new(state.list.filter.clone()).block(block);
    frame.render_widget(search, search_area);

    frame.render_widget(help_line, footer);
}

fn title(state: &State) -> impl Widget {
    let text = match state.context {
        Context::Workspaces => "Workspaces",
        Context::Commands { .. } => "Commands",
    };

    Paragraph::new(text)
}
