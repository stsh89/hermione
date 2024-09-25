use crate::{
    clients,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetWorkspaceModel> {
        self.organizer.delete_command(0, 0)?;
        let workspace = self.organizer.get_workspace(0)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })
    }
}
