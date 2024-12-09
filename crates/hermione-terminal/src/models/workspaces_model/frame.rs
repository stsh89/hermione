use crate::{
    coordinator::Workspace,
    layouts::{SearchListLayout, WideLayout},
    themes::{Theme, HIGHLIGHT_SYMBOL},
    widgets::FormField,
};
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, Text},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

pub struct View<'a> {
    pub theme: &'a Theme,
    pub is_normal_mode: bool,
    pub is_searching: bool,
    pub workspaces: &'a [Workspace],
    pub list_state: ListState,
    pub search: &'a str,
    pub search_character_index: u16,
}

pub fn render(frame: &mut Frame, mut view: View) {
    let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
    let [list_area, input_area] = SearchListLayout::new().areas(main_area);

    let block = Block::default().borders(Borders::all());
    let list = List::new(view.workspaces)
        .block(block)
        .highlight_symbol(HIGHLIGHT_SYMBOL)
        .bg(view.theme.background_color)
        .fg(view.theme.foreground_color)
        .highlight_style(view.theme.highlight_color);

    frame.render_stateful_widget(list, list_area, &mut view.list_state);

    let status_bar = create_status_bar_widget(&view);
    frame.render_widget(status_bar, status_bar_area);

    let search = create_search_widget(&view);
    frame.render_widget(search, input_area);

    if view.is_searching {
        frame.set_cursor_position(Position::new(
            input_area.x + view.search_character_index + 1,
            input_area.y + 1,
        ));
    }
}

fn create_search_widget<'a>(view: &'a View) -> FormField<'a> {
    let mut field = FormField::default()
        .name("Search")
        .value(view.search)
        .set_background_color(view.theme.background_color)
        .set_foreground_color(view.theme.foreground_color);

    if view.is_searching {
        field = field.set_foreground_color(view.theme.input_color);
    }

    field
}

fn create_status_bar_widget<'a>(view: &'a View) -> Paragraph<'a> {
    let mut text = Text::default();

    if view.is_normal_mode {
        text.push_span("Press ");
        text.push_span(Span::from("q ").fg(view.theme.highlight_color));
        text.push_span("to quit, ");
        text.push_span(Span::from("/ ").fg(view.theme.highlight_color));
        text.push_span("to enter search mode, ");
        text.push_span(Span::from("n ").fg(view.theme.highlight_color));
        text.push_span("to create new workspace, ");
        text.push_span(Span::from("e ").fg(view.theme.highlight_color));
        text.push_span("to edit selected workspace");
    } else if view.is_searching {
        text.push_span("Press ");
        text.push_span(Span::from("Esc ").fg(view.theme.highlight_color));
        text.push_span("to discard search, ");
        text.push_span(Span::from("Enter ").fg(view.theme.highlight_color));
        text.push_span("to enter normal mode");
    }

    Paragraph::new(text)
        .bg(view.theme.status_bar_background_color)
        .fg(view.theme.status_bar_foreground_color)
}
