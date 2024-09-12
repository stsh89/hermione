use crate::{
    models::lobby::{Message, Model},
    Result,
};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};

pub struct Controller<'a, B>
where
    B: Backend,
{
    model: Model<'a>,
    terminal: &'a mut Terminal<B>,
    signal: Option<Signal>,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub model: Model<'a>,
    pub terminal: &'a mut Terminal<B>,
}

pub enum Signal {
    EnterCommandCenter(usize),
    NewWorkspaceRequest,
    Exit,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    fn enter_command_center(&mut self) {
        self.signal = Some(Signal::EnterCommandCenter(self.model.workspace().id));
    }

    fn handle_event(&mut self) -> Result<Option<Message>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let message = self.handle_key(key.code)?;

                return Ok(message);
            }
        }

        Ok(None)
    }

    fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<Message>> {
        let message = match key_code {
            KeyCode::Up => Some(Message::SelectPreviousWorkspace),
            KeyCode::Down => Some(Message::SelectNextWorkspace),
            KeyCode::Char('n') => {
                self.request_new_workspace();
                None
            }
            KeyCode::Char('d') => Some(Message::DeleteWorkspace),
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Enter => {
                self.enter_command_center();
                None
            }
            KeyCode::Char('q') => Some(Message::Exit),
            KeyCode::Char('s') => Some(Message::Save),
            _ => None,
        };

        Ok(message)
    }

    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters { model, terminal } = parameters;

        Self {
            model,
            terminal,
            signal: None,
        }
    }

    fn request_new_workspace(&mut self) {
        self.signal = Some(Signal::NewWorkspaceRequest);
    }

    pub fn run(mut self) -> Result<Signal> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model = self.model.update(message)?;
            }

            if self.model.is_exited() {
                self.signal = Some(Signal::Exit);
            }

            if let Some(signal) = self.signal {
                return Ok(signal);
            }
        }
    }
}
