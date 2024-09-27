use crate::{
    clients::memories,
    models::{GetCommandModel, GetCommandModelParameters},
    router::GetCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: GetCommandParameters) -> Result<GetCommandModel> {
        let GetCommandParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        GetCommandModel::new(GetCommandModelParameters { command })
    }
}
