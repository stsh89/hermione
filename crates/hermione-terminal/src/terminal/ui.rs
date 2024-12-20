use crate::program::{Context, State};
use ratatui::{
    layout::{Constraint, Direction, Rect},
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

    frame.render_widget(title(state), header);

    render_content(state, frame, content);

    frame.render_widget(help_line(), footer);
}

fn help_line() -> impl Widget {
    let mut text = Text::default();

    text.push_span("Press ");
    text.push_span(Span::from("q "));
    text.push_span("to quit");

    Paragraph::new(text)
}

fn render_content(state: &State, frame: &mut Frame, area: Rect) {
    match state.context {
        Context::Workspaces | Context::Commands { .. } => render_list(state, frame, area),
        Context::WorkspaceForm { .. } => render_workspace_form(state, frame, area),
    }
}

fn render_workspace_form(state: &State, frame: &mut Frame, area: Rect) {
    let [name_area, location_area] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Max(3)])
        .areas(area);

    let block = Block::default().borders(Borders::ALL).title("Name");
    let paragraph = Paragraph::new(state.form.inputs[0].clone()).block(block);
    frame.render_widget(paragraph, name_area);

    let block = Block::default().borders(Borders::ALL).title("Location");
    let paragraph = Paragraph::new(state.form.inputs[1].clone()).block(block);
    frame.render_widget(paragraph, location_area);
}

fn render_list(state: &State, frame: &mut Frame, area: Rect) {
    let [list_area, search_area] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3), Constraint::Max(3)])
        .areas(area);

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
}

fn title(state: &State) -> impl Widget {
    let text = match state.context {
        Context::Workspaces => "Workspaces",
        Context::Commands { .. } => "Commands",
        Context::WorkspaceForm { workspace_id } => match workspace_id {
            Some(_) => "Edit workspace",
            None => "New workspace",
        },
    };

    Paragraph::new(text)
}
