use crate::{
    coordinator::Coordinator, parameters::workspaces::commands::create::Parameters,
    presenters::command::Presenter, Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.coordinator.workspaces().commands().create(Presenter {
            workspace_id: workspace_id.clone(),
            id: String::new(),
            name,
            program: program.clone(),
        })
    }
}
