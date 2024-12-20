use crate::program::{Context, Render, State};
use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction},
    prelude::CrosstermBackend,
    text::{Span, Text},
    widgets::{Block, Borders, List, ListState, Paragraph, Widget},
    Frame,
};
use std::{
    io::{stdout, Stdout},
    panic,
};

pub struct Terminal {
    inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Render for Terminal {
    fn render(&mut self, state: &State) -> anyhow::Result<()> {
        self.inner.draw(|frame| render(frame, state))?;

        Ok(())
    }
}

pub fn init() -> anyhow::Result<Terminal> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let inner = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;

    Ok(Terminal { inner })
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn restore() -> anyhow::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn render(frame: &mut Frame, state: &State) {
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
