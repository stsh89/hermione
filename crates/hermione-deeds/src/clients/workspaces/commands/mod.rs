pub mod json;

use crate::types::command::Data;

pub trait Operations {
    fn create(&self, data: Data) -> Result<Data, eyre::Report>;
    fn delete(&self, workspace_id: &str, id: &str) -> Result<(), eyre::Report>;
    fn get(&self, workspace_id: &str, id: &str) -> Result<Data, eyre::Report>;
    fn list(&self, workspace_id: &str) -> Result<Vec<Data>, eyre::Report>;
    fn track_execution_time(
        &self,
        workspace_id: &str,
        command_id: &str,
    ) -> Result<Data, eyre::Report>;
    fn update(&self, data: Data) -> Result<Data, eyre::Report>;
}
