use std::rc::Rc;

use crate::database::{CommandRecord, DatabaseProvider, WorkspaceRecord};
use hermione_ops::{
    backup::{ImportCommand, ImportWorkspace, ListAllCommandsInBatches, ListCommands},
    commands::{Command, ListCommandsParameters},
    workspaces::Workspace,
    Result,
};
use rusqlite::{params, types::Value};

const DEFAULT_PAGE_SIZE: u32 = 100;

impl ImportCommand for DatabaseProvider {
    fn import_command(&self, command: Command) -> Result<Command> {
        let record = CommandRecord::from_entity(&command)?;

        self.insert_command(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl ImportWorkspace for DatabaseProvider {
    fn import_workspace(&self, entity: Workspace) -> Result<Workspace> {
        let record = WorkspaceRecord::try_from(&entity)?;

        self.insert_workspace(record).map_err(eyre::Error::new)?;

        Ok(entity)
    }
}

impl ListAllCommandsInBatches for DatabaseProvider {
    async fn list_all_commands_in_batches(
        &self,
        batch_fn: impl Fn(Vec<Command>) -> hermione_ops::Result<()>,
    ) -> Result<()> {
        let mut page_number = 0;

        loop {
            let commands = self.list_commands(ListCommandsParameters {
                page_number,
                page_size: DEFAULT_PAGE_SIZE,
                ids: vec![],
            })?;

            if commands.is_empty() {
                break;
            }

            batch_fn(commands)?;

            page_number += 1;
        }

        Ok(())
    }
}

impl ListCommands for DatabaseProvider {
    fn list_commands(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>> {
        let ListCommandsParameters {
            ids,
            page_number,
            page_size,
        } = parameters;

        let ids: Vec<Vec<u8>> = ids.into_iter().map(|id| id.into_bytes().to_vec()).collect();
        let ids = Rc::new(ids.into_iter().map(Value::from).collect::<Vec<Value>>());
        let empty_ids = if ids.is_empty() { 1 } else { 0 };

        let mut statement = self
            .connection()
            .prepare(
                "SELECT
                    id,
                    last_execute_time,
                    name,
                    program,
                    workspace_id
                FROM commands
                WHERE (id IN rarray(?1) OR 1 == ?2)
                ORDER BY program ASC
                LIMIT ?3 OFFSET ?4",
            )
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![ids, empty_ids, page_size, page_number * page_size],
                CommandRecord::from_row,
            )
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records
            .into_iter()
            .map(CommandRecord::load_entity)
            .collect();

        Ok(entities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hermione_ops::{
        commands::{CreateCommand, NewCommandParameters},
        workspaces::{CreateWorkspace, NewWorkspaceParameters},
    };
    use std::path::Path;

    const TEST_DB_FILE_PATH: &str = "tests/assets/hermione_test.db3";

    fn prepare_test_db() -> Result<DatabaseProvider> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(eyre::Error::new)?;
        let path = Path::new(&manifest_dir).join(TEST_DB_FILE_PATH);

        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        let provider = DatabaseProvider::new(&path).map_err(eyre::Error::new)?;

        Ok(provider)
    }

    #[test]
    fn test_list_commands() -> Result<()> {
        let db = prepare_test_db()?;

        let workspace = db.create_workspace(Workspace::new(NewWorkspaceParameters {
            name: "Test workspace".into(),
            location: None,
        }))?;

        let command1 = db.create_command(Command::new(NewCommandParameters {
            name: "Cmd 1".into(),
            program: "Prg 1".into(),
            workspace_id: workspace.try_id()?,
        }))?;

        let command2 = db.create_command(Command::new(NewCommandParameters {
            name: "Cmd 2".into(),
            program: "Prg 2".into(),
            workspace_id: workspace.try_id()?,
        }))?;

        let commands = db.list_commands(ListCommandsParameters {
            page_number: 0,
            page_size: 10,
            ids: vec![command2.try_id()?],
        })?;

        assert_eq!(commands.len(), 1);

        let command = commands.into_iter().next().unwrap();

        assert_eq!(command, command2);

        let commands = db.list_commands(ListCommandsParameters {
            page_number: 0,
            page_size: 10,
            ids: vec![command2.try_id()?, command1.try_id()?],
        })?;

        assert_eq!(commands.len(), 2);

        let commands = db.list_commands(ListCommandsParameters {
            page_number: 0,
            page_size: 10,
            ids: vec![],
        })?;

        assert_eq!(commands.len(), 2);

        Ok(())
    }
}
