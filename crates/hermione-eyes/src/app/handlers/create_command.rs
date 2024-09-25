use crate::{
    clients::{self, organizer::CommandParameters},
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::CreateCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
    pub parameters: CreateCommandParameters,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetWorkspaceModel> {
        let CreateCommandParameters { name, program } = self.parameters;
        self.organizer.add_command(CommandParameters {
            workspace_number: 0,
            name: name.clone(),
            program: program.clone(),
        })?;

        let workspace = self.organizer.get_workspace(0)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })
    }
}
