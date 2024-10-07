use crate::{
    clients, parameters::workspaces::commands::create::Parameters, presenters::command::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a clients::memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.memories.create_command(Presenter {
            workspace_id: workspace_id.clone(),
            id: String::new(),
            name,
            program: program.clone(),
        })
    }
}
