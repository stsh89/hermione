pub mod commands;
pub mod json;

use crate::types::workspace::Data;

pub trait Operations {
    fn create(&self, data: Data) -> Result<Data, eyre::Report>;
    fn delete(&self, id: &str) -> Result<(), eyre::Report>;
    fn get(&self, id: &str) -> Result<Data, eyre::Report>;
    fn list(&self) -> Result<Vec<Data>, eyre::Report>;
    fn track_access_time(&self, id: &str) -> Result<Data, eyre::Report>;
    fn update(&self, data: Data) -> Result<Data, eyre::Report>;
}
