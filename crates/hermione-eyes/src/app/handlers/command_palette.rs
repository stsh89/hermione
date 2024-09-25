use crate::{
    models::{Action, CommandPaletteModel, CommandPaletteModelParameters},
    router::{CommandPaletteParameters, Router},
    Result,
};

pub struct Handler {
    pub parameters: CommandPaletteParameters,
    pub route: Router,
}

impl Handler {
    pub fn handle(self) -> Result<CommandPaletteModel> {
        let CommandPaletteParameters { actions: commands } = self.parameters;
        let commands = commands
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<Action>>>()?;

        let model = CommandPaletteModel::new(CommandPaletteModelParameters {
            commands,
            back: self.route,
        });

        Ok(model)
    }
}
