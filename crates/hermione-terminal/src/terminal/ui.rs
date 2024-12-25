use crate::program_lib::{Context, Mode, NoticeKind, State};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Clear, List, ListState, Paragraph, Widget, Wrap},
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

    if let Some(notice) = &state.notice {
        let title = match notice.kind {
            NoticeKind::Success => "Success",
            NoticeKind::Error => "Error",
        };

        let popup_area = popup_area(content, 50, 50);
        let block = Block::default().borders(Borders::all()).title(title);
        let paragraph = Paragraph::new(notice.message.as_str())
            .wrap(Wrap { trim: false })
            .block(block);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);
    }
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
        Context::CommandForm { .. } => render_command_form(state, frame, area),
        Context::NotionBackupCredentialsForm => render_notion_form(state, frame, area),
    }
}

fn render_workspace_form(state: &State, frame: &mut Frame, area: Rect) {
    let [name_area, location_area] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Max(3)])
        .areas(area);

    let mut block = Block::default().borders(Borders::ALL).title("Name");
    if matches!(state.mode, Mode::Input) && state.form.cursor == 0 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let paragraph = Paragraph::new(state.form.inputs[0].as_str()).block(block);
    frame.render_widget(paragraph, name_area);

    let mut block = Block::default().borders(Borders::ALL).title("Location");
    if matches!(state.mode, Mode::Input) && state.form.cursor == 1 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let paragraph = Paragraph::new(state.form.inputs[1].as_str()).block(block);
    frame.render_widget(paragraph, location_area);
}

fn render_command_form(state: &State, frame: &mut Frame, area: Rect) {
    let [name_area, program_area] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
        .areas(area);

    let mut block = Block::default().borders(Borders::ALL).title("Name");
    if matches!(state.mode, Mode::Input) && state.form.cursor == 0 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let paragraph = Paragraph::new(state.form.inputs[0].as_str()).block(block);
    frame.render_widget(paragraph, name_area);

    let mut block = Block::default().borders(Borders::ALL).title("Program");
    if matches!(state.mode, Mode::Input) && state.form.cursor == 1 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let paragraph = Paragraph::new(state.form.inputs[1].as_str()).block(block);
    frame.render_widget(paragraph, program_area);
}

fn render_notion_form(state: &State, frame: &mut Frame, area: Rect) {
    let [api_key_area, commands_database_id_area, workspaces_database_id_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Max(3),
        ])
        .areas(area);

    let mut block = Block::default().borders(Borders::ALL).title("Api key");
    if matches!(state.mode, Mode::Input) && state.form.cursor == 0 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let paragraph = Paragraph::new(state.form.inputs[0].as_str()).block(block);
    frame.render_widget(paragraph, api_key_area);

    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Commands database id");

    if matches!(state.mode, Mode::Input) && state.form.cursor == 1 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }
    let paragraph = Paragraph::new(state.form.inputs[1].as_str()).block(block);
    frame.render_widget(paragraph, commands_database_id_area);

    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Workspaces database id");
    if matches!(state.mode, Mode::Input) && state.form.cursor == 2 {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let paragraph = Paragraph::new(state.form.inputs[2].as_str()).block(block);
    frame.render_widget(paragraph, workspaces_database_id_area);
}

fn render_list(state: &State, frame: &mut Frame, area: Rect) {
    let [list_area, search_area] = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3), Constraint::Max(3)])
        .areas(area);

    let mut block = Block::default().borders(Borders::ALL);
    if matches!(state.mode, Mode::Normal) {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let list = List::new(
        state
            .list
            .items
            .iter()
            .map(|item| item.text.as_str())
            .collect::<Vec<_>>(),
    )
    .block(block)
    .highlight_symbol(">");
    let mut list_state = ListState::default();

    if !state.list.items.is_empty() {
        list_state.select(Some(state.list.cursor));
    }

    frame.render_stateful_widget(list, list_area, &mut list_state);

    let mut block = Block::default().borders(Borders::ALL);
    if matches!(state.mode, Mode::Input) {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let search = Paragraph::new(state.list.filter.as_str()).block(block);
    frame.render_widget(search, search_area);
}

fn title(state: &State) -> impl Widget {
    let text = match state.context {
        Context::Workspaces => "Workspaces",
        Context::Commands { .. } => "Commands",
        Context::WorkspaceForm => match state.workspace_id {
            Some(_) => "Edit workspace",
            None => "New workspace",
        },
        Context::CommandForm => match state.command_id {
            Some(_) => "Edit command",
            None => "New command",
        },
        Context::NotionBackupCredentialsForm => "Notion",
    };

    Paragraph::new(text)
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);

    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    area
}
