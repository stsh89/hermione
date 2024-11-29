use crate::{
    definitions::WorkspaceId,
    services::{DeleteWorkspaceCommands, StorageService},
    Result,
};

pub struct DeleteCommandsOperation<'a, DWC>
where
    DWC: StorageService,
{
    pub delete_workspace_commands: &'a DWC,
}

pub struct DeleteCommandsParameters {
    pub delete_attribute: CommandsDeleteAttribute,
}

pub enum CommandsDeleteAttribute {
    WorkspaceId(WorkspaceId),
}

impl<'a, DWC> DeleteCommandsOperation<'a, DWC>
where
    DWC: DeleteWorkspaceCommands,
{
    pub fn delete_workspace_commands(&self, id: WorkspaceId) -> Result<()> {
        self.delete_workspace_commands.delete_workspace_commands(id)
    }

    pub fn execute(&self, parameters: DeleteCommandsParameters) -> Result<()> {
        let DeleteCommandsParameters { delete_attribute } = parameters;

        use CommandsDeleteAttribute as Attr;

        match delete_attribute {
            Attr::WorkspaceId(workspace_id) => self.delete_workspace_commands(workspace_id),
        }
    }
}
