use crate::{
    clients,
    models::{
        GetCommandModel, GetCommandModelParameters, GetWorkspaceModel, GetWorkspaceModelParameters,
    },
    router::GetCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
    pub parameters: GetCommandParameters,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetCommandModel> {
        let GetCommandParameters { number } = self.parameters;

        let command = self.organizer.get_command(0, number)?;

        self.organizer.promote_command(0, command.number)?;

        let model = GetCommandModel::new(GetCommandModelParameters { command });

        Ok(model)
    }
}
