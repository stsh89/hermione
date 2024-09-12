use crate::{
    data::Command,
    models::command_center::{Message, Model},
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
    ExecuteCommand(usize),
    NewCommandRequest,
    Exit,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    fn execute_command(&mut self, command: &Command) {
        self.signal = Some(Signal::ExecuteCommand(command.id))
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<Message>> {
        let message = match key_code {
            KeyCode::Up => Some(Message::SelectPreviousCommand),
            KeyCode::Down => Some(Message::SelectNextCommand),
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Char(c) if self.model.is_editing() => Some(Message::EnterChar(c)),
            KeyCode::Left if self.model.is_editing() => Some(Message::MoveCusorLeft),
            KeyCode::Right if self.model.is_editing() => Some(Message::MoveCusorRight),
            KeyCode::Backspace if self.model.is_editing() => Some(Message::DeleteChar),
            KeyCode::Char('n') => {
                self.request_new_command();
                None
            }
            KeyCode::Char('d') if self.model.is_selecting() => Some(Message::DeleteCommand),
            KeyCode::Enter => {
                if let Some(command) = self.model.command() {
                    self.execute_command(&Command {
                        id: command.id,
                        name: command.name.clone(),
                        program: command.program.clone(),
                    });
                    None
                } else {
                    None
                }
            }
            KeyCode::Char('s') if self.model.is_selecting() => Some(Message::ActivateSearchBar),
            _ => None,
        };

        Ok(message)
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

    pub fn new(params: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters { model, terminal } = params;

        Self {
            model,
            terminal,
            signal: None,
        }
    }

    fn request_new_command(&mut self) {
        self.signal = Some(Signal::NewCommandRequest);
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
