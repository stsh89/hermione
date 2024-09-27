use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

pub struct Client {
    commands_path: PathBuf,
    workspaces_path: PathBuf,
}

impl Client {
    pub fn new(path: PathBuf) -> Result<Self, anyhow::Error> {
        let commands_path = path.join("commands.json");
        let workspaces_path = path.join("workspaces.json");

        for path in &[&commands_path, &workspaces_path] {
            if path.exists() {
                continue;
            }

            let mut file = std::fs::File::create(path)?;
            file.write_all(b"[]")?;
        }

        Ok(Self {
            commands_path,
            workspaces_path,
        })
    }

    fn commands(&self) -> Result<Vec<commands::Record>, eyre::Report> {
        let mut commands: Vec<commands::Record> = self.list_records(&self.commands_path)?;

        commands.sort_by(|a, b| a.last_execute_time.cmp(&b.last_execute_time).reverse());

        Ok(commands)
    }

    fn workspaces(&self) -> Result<Vec<workspaces::Record>, eyre::Report> {
        self.list_records(&self.workspaces_path)
    }

    fn save_commands(&self, records: Vec<commands::Record>) -> Result<(), eyre::Report> {
        self.save_records(records, &self.commands_path)
    }

    fn save_workspaces(&self, records: Vec<workspaces::Record>) -> Result<(), eyre::Report> {
        self.save_records(records, &self.workspaces_path)
    }

    fn list_records<T, P>(&self, path: P) -> Result<Vec<T>, eyre::Report>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let records = serde_json::from_reader(reader)?;

        Ok(records)
    }

    fn save_records<S, P>(&self, records: Vec<S>, path: P) -> Result<(), eyre::Report>
    where
        P: AsRef<Path>,
        S: Serialize,
    {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(eyre::Report::new)?;

        serde_json::to_writer(&mut file, &records)?;

        Ok(())
    }
}

mod workspaces {
    use std::str::FromStr;

    use super::Client;
    use hermione_memories::{
        operations::workspaces::{create, delete, get, list, update},
        types::{
            shared::{DateTime, Error, Id},
            workspace::{Entity, LoadParameters, Location, Name},
        },
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Record {
        id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        last_load_time: Option<chrono::DateTime<chrono::Utc>>,

        location: String,
        name: String,
    }

    impl create::Create for Client {
        fn create(&self, mut workspace: Entity) -> Result<Entity, Error> {
            let mut records = self.workspaces()?;
            workspace.set_id(Id::generate())?;

            records.push(Record {
                id: unsafe { workspace.id().to_string() },
                last_load_time: workspace.last_load_time().map(|time| time.to_chrono()),
                location: workspace
                    .location()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                name: workspace.name().to_string(),
            });

            self.save_workspaces(records)?;

            Ok(workspace)
        }
    }

    impl delete::Delete for Client {
        fn delete(&self, id: Id) -> Result<(), Error> {
            let mut records = self.workspaces()?;
            let id = id.to_string();

            records.retain(|record| record.id != id);

            self.save_workspaces(records)?;

            Ok(())
        }
    }

    impl get::Get for Client {
        fn get(&self, id: Id) -> Result<Entity, Error> {
            let id = id.to_string();
            let record = self
                .workspaces()?
                .into_iter()
                .find(|record| record.id == id)
                .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

            let record = record.try_into()?;

            Ok(record)
        }
    }

    impl list::List for Client {
        fn list(&self) -> Result<Vec<Entity>, Error> {
            let records = self
                .workspaces()?
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(records)
        }
    }

    impl update::Update for Client {
        fn update(&self, workspace: Entity) -> Result<Entity, Error> {
            let mut records = self.workspaces()?;
            let id = unsafe { workspace.id().to_string() };

            let record = records
                .iter_mut()
                .find(|record| record.id == id)
                .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

            record.name = workspace.name().to_string();
            record.location = workspace
                .location()
                .map(ToString::to_string)
                .unwrap_or_default();

            self.save_workspaces(records)?;

            Ok(workspace)
        }
    }

    impl TryFrom<Record> for Entity {
        type Error = eyre::Report;

        fn try_from(record: Record) -> Result<Self, Self::Error> {
            let command = Entity::load(LoadParameters {
                id: Id::from_str(&record.id)?,
                last_load_time: record.last_load_time.map(DateTime::from_chrono),
                location: Some(Location::new(record.location)),
                name: Name::new(record.name),
            });

            Ok(command)
        }
    }
}

mod commands {
    use std::str::FromStr;

    use super::Client;
    use hermione_memories::{
        operations::workspaces::commands::{
            create, delete, get, list, track_execution_time, update,
        },
        types::{
            command::{Entity, LoadParameters, Name, Program, WorkspaceScopeId},
            shared::{DateTime, Error, Id},
        },
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Record {
        id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_execute_time: Option<chrono::DateTime<chrono::Utc>>,

        name: String,
        program: String,
        workspace_id: String,
    }

    impl create::Create for Client {
        fn create(&self, mut command: Entity) -> Result<Entity, Error> {
            let mut records = self.commands()?;
            command.set_id(Id::generate())?;

            records.push(Record {
                last_execute_time: command.last_execute_time().map(|time| time.to_chrono()),
                id: unsafe { command.id().to_string() },
                name: command.name().to_string(),
                program: command.program().to_string(),
                workspace_id: command.workspace_id().to_string(),
            });

            self.save_commands(records)?;

            Ok(command)
        }
    }

    impl delete::Delete for Client {
        fn delete(&self, id: WorkspaceScopeId) -> Result<(), Error> {
            let WorkspaceScopeId {
                workspace_id,
                command_id,
            } = id;
            let command_id = command_id.to_string();
            let workspace_id = workspace_id.to_string();

            let mut records = self.commands()?;

            records
                .retain(|record| !(record.id == command_id && record.workspace_id == workspace_id));

            self.save_commands(records)?;

            Ok(())
        }
    }

    impl get::Get for Client {
        fn get(&self, id: WorkspaceScopeId) -> Result<Entity, Error> {
            let WorkspaceScopeId {
                workspace_id,
                command_id,
            } = id;
            let command_id = command_id.to_string();
            let workspace_id = workspace_id.to_string();

            let record = self
                .commands()?
                .into_iter()
                .find(|record| record.id == command_id && record.workspace_id == workspace_id)
                .ok_or_else(|| {
                    eyre::eyre!(
                        "Command with id {} and workspace id {} not found",
                        command_id,
                        workspace_id
                    )
                })?;

            let record = record.try_into()?;

            Ok(record)
        }
    }

    impl list::List for Client {
        fn list(&self, workspace_id: Id) -> Result<Vec<Entity>, Error> {
            let workspace_id = workspace_id.to_string();

            let records = self
                .commands()?
                .into_iter()
                .filter(|record| record.workspace_id == workspace_id)
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(records)
        }
    }

    impl track_execution_time::Track for Client {
        fn track(&self, command: Entity) -> Result<Entity, Error> {
            let mut records = self.commands()?;
            let command_id = unsafe { command.id().to_string() };
            let workspace_id = command.workspace_id().to_string();

            let record = records
                .iter_mut()
                .find(|record| record.id == command_id && record.workspace_id == workspace_id)
                .ok_or_else(|| {
                    eyre::eyre!(
                        "Command with id {} and workspace id {} not found",
                        command_id,
                        workspace_id
                    )
                })?;

            record.last_execute_time = Some(chrono::Utc::now());

            let command = Entity::load(LoadParameters {
                last_execute_time: record.last_execute_time.map(DateTime::from_chrono),
                id: Id::from_str(&record.id)?,
                name: Name::new(record.name.clone()),
                program: Program::new(record.program.clone()),
                workspace_id: Id::from_str(&record.workspace_id)?,
            });

            self.save_commands(records)?;

            Ok(command)
        }
    }

    impl update::Update for Client {
        fn update(&self, command: Entity) -> Result<Entity, Error> {
            let mut records = self.commands()?;
            let command_id = unsafe { command.id().to_string() };
            let workspace_id = command.workspace_id().to_string();

            let record = records
                .iter_mut()
                .find(|record| record.id == command_id && record.workspace_id == workspace_id)
                .ok_or_else(|| {
                    eyre::eyre!(
                        "Command with id {} and workspace id {} not found",
                        command_id,
                        workspace_id
                    )
                })?;

            record.name = command.name().to_string();
            record.program = command.program().to_string();

            self.save_commands(records)?;

            Ok(command)
        }
    }

    impl TryFrom<Record> for Entity {
        type Error = eyre::Report;

        fn try_from(record: Record) -> Result<Self, Self::Error> {
            let command = Entity::load(LoadParameters {
                last_execute_time: record.last_execute_time.map(DateTime::from_chrono),
                id: Id::from_str(&record.id)?,
                name: Name::new(record.name.clone()),
                program: Program::new(record.program.clone()),
                workspace_id: Id::from_str(&record.workspace_id)?,
            });

            Ok(command)
        }
    }
}
