use super::handle_event;
use crate::{
    key_mappings::new_workspace_key_mapping,
    models::new_workspace::{Model, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Controller<'a, B>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters { terminal } = parameters;

        Self { terminal }
    }

    pub fn run(self) -> Result<Signal> {
        let mut model = Model::new();

        while model.is_running() {
            self.terminal.draw(|frame| model.view(frame))?;

            if let Some(message) = handle_event(new_workspace_key_mapping)? {
                model = model.update(message);
            }
        }

        Ok(unsafe { model.signal() })
    }
}
