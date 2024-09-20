use crate::{
    controllers::editor::{Controller, ControllerParameters},
    entities::Workspace,
    models::workspace_editor::{Model, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct WorkspaceEditor<'a, B>
where
    B: Backend,
{
    pub workspace: Option<&'a Workspace>,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> WorkspaceEditor<'a, B>
where
    B: Backend,
{
    pub fn enter(self) -> Result<Signal> {
        Controller::new(ControllerParameters {
            model: Model::from_workspace(self.workspace)?,
            terminal: self.terminal,
        })
        .run()
    }
}
