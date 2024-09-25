use crate::{
    clients::{executor, organizer},
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::ExecuteCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub parameters: ExecuteCommandParameters,
    pub organizer: &'a mut organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetWorkspaceModel> {
        let ExecuteCommandParameters { number } = self.parameters;
        let command = self.organizer.get_command(0, number)?;
        let workspace = self.organizer.get_workspace(0)?;

        let executor = executor::Client::new(&command, &workspace.location);
        executor.execute()?;

        self.organizer.promote_command(0, number)?;
        let workspace = self.organizer.get_workspace(0)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })
    }
}
