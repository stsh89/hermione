use crate::{
    clients,
    models::{GetCommandModel, GetCommandModelParameters},
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

        GetCommandModel::new(GetCommandModelParameters { command })
    }
}
