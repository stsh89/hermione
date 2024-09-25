use crate::{
    clients::{self, organizer::CommandParameters},
    models::{GetCommandModel, GetCommandModelParameters},
    router::UpdateCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
    pub parameters: UpdateCommandParameters,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetCommandModel> {
        let UpdateCommandParameters { name, program } = self.parameters;
        self.organizer.update_command(
            0,
            CommandParameters {
                workspace_number: 0,
                name,
                program,
            },
        )?;

        let command = self.organizer.get_command(0, 0)?;

        let model = GetCommandModel::new(GetCommandModelParameters { command })?;

        Ok(model)
    }
}
