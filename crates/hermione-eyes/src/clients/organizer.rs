use crate::entities::{Command, Workspace};
use hermione_memories::{
    operations::{
        create_command, create_workspace, delete_command, delete_workspace, get_command,
        get_workspace, list_workspaces, update_command, update_workspace,
    },
    types::{
        command,
        shared::{DateTime, Error, Id},
        workspace,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::BufReader,
    str::FromStr,
};

pub struct Client {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
struct WorkspaceRecord {
    commands: Vec<CommandRecord>,
    id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_load_time: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct CommandRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_execute_time: Option<chrono::DateTime<chrono::Utc>>,
    id: String,
    name: String,
    program: String,
}

impl create_command::Create for Client {
    fn create(
        &self,
        workspace_id: Id,
        mut command: command::Entity,
    ) -> Result<command::Entity, Error> {
        let mut records = self.list_workspace_records()?;
        command.set_id(Id::generate())?;

        let record = records
            .iter_mut()
            .find(|record| record.id == workspace_id.to_string())
            .ok_or(eyre::eyre!("Workspace with id {} not found", workspace_id))?;

        record.commands.push(CommandRecord {
            last_execute_time: command.last_execute_time().map(|time| time.to_chrono()),
            id: command.id()?.to_string(),
            name: command.name().to_string(),
            program: command.program().to_string(),
        });

        self.save_workspace_records(records)?;

        Ok(command)
    }
}

impl delete_command::Delete for Client {
    fn delete(&self, workspace_id: Id, command_id: Id) -> Result<(), Error> {
        let mut records = self.list_workspace_records().map_err(eyre::Report::from)?;

        let record = records
            .iter_mut()
            .find(|record| record.id == workspace_id.to_string())
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", workspace_id))?;

        record
            .commands
            .retain(|record| record.id != command_id.to_string());

        self.save_workspace_records(records)
            .map_err(eyre::Report::from)?;

        Ok(())
    }
}

impl get_command::Get for Client {
    fn get(&self, workspace_id: Id, command_id: Id) -> Result<command::Entity, Error> {
        let command = self
            .list_workspace_records()?
            .into_iter()
            .find(|record| record.id == workspace_id.to_string())
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", workspace_id))?
            .commands
            .into_iter()
            .find(|record| record.id == command_id.to_string())
            .ok_or_else(|| eyre::eyre!("Command with id {} not found", command_id))?;

        Ok(command::Entity::load(command::LoadParameters {
            last_execute_time: command.last_execute_time.map(DateTime::from_chrono),
            id: Id::from_str(&command.id)?,
            name: command::Name::new(command.name.clone()),
            program: command::Program::new(command.program.clone()),
        }))
    }
}

impl update_command::Update for Client {
    fn update(&self, workspace_id: Id, command: &command::Entity) -> Result<(), Error> {
        let mut records = self.list_workspace_records()?;
        let record = records
            .iter_mut()
            .find(|record| record.id == workspace_id.to_string())
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", workspace_id))?;

        let command_id = command.id()?.to_string();

        let update = record
            .commands
            .iter_mut()
            .find(|record| record.id == command_id)
            .ok_or_else(|| eyre::eyre!("Command with id {} not found", command_id))?;

        update.name = command.name().to_string();
        update.program = command.program().to_string();

        self.save_workspace_records(records)?;
        Ok(())
    }
}

impl delete_workspace::Delete for Client {
    fn delete(&self, id: Id) -> Result<(), Error> {
        let mut workspaces = self.list_workspace_records()?;

        workspaces.retain(|workspace| workspace.id != id.to_string());

        self.save_workspace_records(workspaces)?;

        Ok(())
    }
}

impl get_workspace::Get for Client {
    fn get(&self, id: Id) -> Result<workspace::Entity, Error> {
        let records = self.list_workspace_records()?;

        let record = records
            .into_iter()
            .find(|record| record.id == id.to_string())
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        Ok(record.try_into()?)
    }
}

impl list_workspaces::List for Client {
    fn list(&self) -> Result<Vec<workspace::Entity>, Error> {
        let records = self.list_workspace_records()?;

        let workspaces = records
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(workspaces)
    }
}

impl create_workspace::Create for Client {
    fn create(&self, mut workspace: workspace::Entity) -> Result<workspace::Entity, Error> {
        tracing::info!("Create new workspace");

        let mut records = self.list_workspace_records()?;
        let id = Id::generate();

        workspace.set_id(id)?;

        records.push(WorkspaceRecord {
            commands: vec![],
            id: workspace.id()?.to_string(),
            last_load_time: workspace.last_load_time().map(|time| time.to_chrono()),
            location: workspace.location().map(ToString::to_string),
            name: workspace.name().to_string(),
        });

        self.save_workspace_records(records)?;

        Ok(workspace)
    }
}

impl update_workspace::Update for Client {
    fn update(&self, workspace: &workspace::Entity) -> Result<(), Error> {
        let mut records = self.list_workspace_records()?;
        let id = workspace.id()?.to_string();

        let record = records
            .iter()
            .position(|record| record.id == id)
            .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        records[record] = WorkspaceRecord {
            commands: workspace
                .commands()
                .iter()
                .map(|command| {
                    Ok(CommandRecord {
                        last_execute_time: command.last_execute_time().map(|time| time.to_chrono()),
                        id: command.id()?.to_string(),
                        name: command.name().to_string(),
                        program: command.program().to_string(),
                    })
                })
                .collect::<Result<Vec<CommandRecord>, Error>>()?,
            id: workspace.id()?.to_string(),
            last_load_time: workspace.last_load_time().map(|time| time.to_chrono()),
            location: workspace.location().map(ToString::to_string),
            name: workspace.name().to_string(),
        };

        self.save_workspace_records(records)?;

        Ok(())
    }
}

impl Client {
    fn list_workspace_records(&self) -> Result<Vec<WorkspaceRecord>, eyre::Report> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let records = serde_json::from_reader(reader)?;

        Ok(records)
    }

    fn save_workspace_records(&self, workspaces: Vec<WorkspaceRecord>) -> Result<(), eyre::Report> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .map_err(eyre::Report::new)?;

        serde_json::to_writer(&mut file, &workspaces).map_err(eyre::Report::new)
    }

    pub fn create_command(&self, command: Command) -> crate::Result<()> {
        use create_command::Create;

        self.create(
            Id::from_str(&command.workspace_id)?,
            command::Entity::new(command::NewParameters {
                name: command::Name::new(command.name),
                program: command::Program::new(command.program),
            }),
        )?;

        Ok(())
    }

    pub fn delete_command(&self, workspace_id: &str, command_id: &str) -> crate::Result<()> {
        use delete_command::Delete;

        self.delete(Id::from_str(workspace_id)?, Id::from_str(command_id)?)?;

        Ok(())
    }

    pub fn delete_workspace(&self, id: &str) -> crate::Result<()> {
        use delete_workspace::Delete;

        self.delete(Id::from_str(id)?)?;

        Ok(())
    }

    pub fn get_command(&self, workspace_id: &str, command_id: &str) -> crate::Result<Command> {
        use get_command::Get;

        let command = self.get(Id::from_str(workspace_id)?, Id::from_str(command_id)?)?;

        Ok(Command {
            workspace_id: workspace_id.to_string(),
            id: Some(command.id()?.to_string()),
            name: command.name().to_string(),
            program: command.program().to_string(),
        })
    }

    pub fn get_workspace(&self, id: &str) -> crate::Result<Workspace> {
        use get_workspace::Get;

        let workspace = self.get(Id::from_str(id)?)?.try_into()?;

        Ok(workspace)
    }

    pub fn list_workspaces(&self) -> crate::Result<Vec<Workspace>> {
        use list_workspaces::List;

        let workspaces = self
            .list()?
            .into_iter()
            .map(TryInto::<Workspace>::try_into)
            .collect::<crate::Result<Vec<Workspace>>>()?;

        Ok(workspaces)
    }

    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn create_workspace(&self, mut new_workspace: Workspace) -> crate::Result<Workspace> {
        use create_workspace::Create;

        let workspace = self.create(workspace::Entity::new(workspace::NewParameters {
            name: workspace::Name::new(new_workspace.name.clone()),
            location: Some(workspace::Location::new(new_workspace.location.clone())),
        })?)?;

        new_workspace.id = Some(workspace.id()?.to_string());

        Ok(new_workspace)
    }

    pub fn update_workspace(&self, update: &Workspace) -> crate::Result<()> {
        use get_workspace::Get;
        use update_workspace::Update;

        let mut workspace = self.get(Id::from_str(update.id())?)?;
        workspace.rename(workspace::Name::new(update.name.clone()));
        workspace.change_location(workspace::Location::new(update.location.clone()));

        self.update(&workspace)?;

        Ok(())
    }

    pub fn update_command(&self, update: &Command) -> crate::Result<()> {
        use get_command::Get;
        use update_command::Update;

        let workspace_id = Id::from_str(&update.workspace_id)?;
        let mut command = self.get(workspace_id, Id::from_str(update.id())?)?;
        command.change_name(command::Name::new(update.name.clone()));
        command.change_program(command::Program::new(update.program.clone()));

        self.update(workspace_id, &command)?;

        Ok(())
    }
}

impl TryFrom<WorkspaceRecord> for workspace::Entity {
    type Error = eyre::Report;

    fn try_from(record: WorkspaceRecord) -> Result<workspace::Entity, Self::Error> {
        Ok(workspace::Entity::load(workspace::LoadParameters {
            commands: record
                .commands
                .into_iter()
                .map(|record| record.try_into())
                .collect::<Result<Vec<command::Entity>, Self::Error>>()?,
            id: Id::from_str(&record.id)?,
            last_load_time: record.last_load_time.map(DateTime::from_chrono),
            location: record.location.map(workspace::Location::new),
            name: workspace::Name::new(record.name),
        }))
    }
}

impl TryFrom<CommandRecord> for command::Entity {
    type Error = eyre::Report;

    fn try_from(record: CommandRecord) -> Result<Self, Self::Error> {
        let command = command::Entity::load(command::LoadParameters {
            last_execute_time: record.last_execute_time.map(DateTime::from_chrono),
            id: Id::from_str(&record.id)?,
            name: command::Name::new(record.name),
            program: command::Program::new(record.program),
        });

        Ok(command)
    }
}

impl TryFrom<workspace::Entity> for Workspace {
    type Error = crate::Error;

    fn try_from(value: workspace::Entity) -> Result<Self, Self::Error> {
        Ok(Workspace {
            name: value.name().to_string(),
            commands: value
                .commands()
                .iter()
                .map(|command| {
                    Ok(Command {
                        workspace_id: value.id()?.to_string(),
                        id: Some(command.id()?.to_string()),
                        name: command.name().to_string(),
                        program: command.program().to_string(),
                    })
                })
                .collect::<Result<Vec<Command>, crate::Error>>()?,
            location: value
                .location()
                .map(ToString::to_string)
                .unwrap_or_default(),
            id: value.id().map(|id| id.to_string()).ok(),
        })
    }
}
