use crate::{
    controllers::editor::{Controller, ControllerParameters},
    entities::Command,
    models::command_editor::{Model, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct CommandEditor<'a, B>
where
    B: Backend,
{
    pub command: Option<&'a Command>,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> CommandEditor<'a, B>
where
    B: Backend,
{
    pub fn enter(self) -> Result<Signal> {
        Controller::new(ControllerParameters {
            model: Model::from_command(self.command)?,
            terminal: self.terminal,
        })
        .run()
    }
}
