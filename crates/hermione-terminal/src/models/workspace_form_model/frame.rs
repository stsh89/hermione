use crate::{layouts::WideLayout, themes::Theme, widgets::FormField};
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    style::Stylize,
    text::{Span, Text},
    widgets::Paragraph,
    Frame,
};

pub struct View<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub name_character_index: u16,
    pub location_character_index: u16,
    pub theme: &'a Theme,
    pub name_is_active: bool,
    pub location_is_active: bool,
    pub is_normal_mode: bool,
    pub is_input_mode: bool,
}

pub fn render(frame: &mut Frame, view: View) {
    let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
    let [name_area, location_area] = create_input_areas(main_area);

    let name = create_name_widget(&view);
    let location = create_location_widget(&view);
    let status_bar = create_status_bar_widget(&view);

    frame.render_widget(name, name_area);
    frame.render_widget(location, location_area);
    frame.render_widget(status_bar, status_bar_area);

    if view.name_is_active {
        frame.set_cursor_position(Position::new(
            name_area.x + view.name_character_index + 1,
            name_area.y + 1,
        ));
    }

    if view.location_is_active {
        frame.set_cursor_position(Position::new(
            location_area.x + view.location_character_index + 1,
            location_area.y + 1,
        ));
    }
}

fn create_input_areas(main_area: Rect) -> [Rect; 2] {
    ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
        .areas(main_area)
}

fn create_location_widget<'a>(view: &'a View) -> FormField<'a> {
    let mut field = FormField::default()
        .name("Location")
        .value(view.location)
        .set_background_color(view.theme.background_color)
        .set_foreground_color(view.theme.foreground_color);

    if view.location_is_active {
        field = field.set_foreground_color(view.theme.input_color);
    }

    field
}

fn create_name_widget<'a>(view: &'a View) -> FormField<'a> {
    let mut field = FormField::default()
        .name("Name")
        .value(view.name)
        .set_background_color(view.theme.background_color)
        .set_foreground_color(view.theme.foreground_color);

    if view.name_is_active {
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
        text.push_span(Span::from("i ").fg(view.theme.highlight_color));
        text.push_span("to enter input mode");
    } else if view.is_input_mode {
        text.push_span("Press ");
        text.push_span(Span::from("Esc ").fg(view.theme.highlight_color));
        text.push_span("to exit input mode");
    }

    Paragraph::new(text)
        .bg(view.theme.status_bar_background_color)
        .fg(view.theme.status_bar_foreground_color)
}
