use chrono::{DateTime, NaiveDateTime, Utc};
use hermione_nexus::definitions::{CommandId, WorkspaceId};
use uuid::Uuid;

#[macro_export]
macro_rules! table { ($($toml:tt)*) => { Table(&toml::toml!($($toml)*)) }; }

pub struct Table<'a>(pub &'a toml::Table);

impl<'a> Table<'a> {
    pub fn get_array(&self, key: &str) -> Vec<Table> {
        let array_of_values = self
            .0
            .get(key)
            .and_then(|value| value.as_array())
            .unwrap_or_else(|| panic!("Table should have array value for `{key}` key"));

        array_of_values
            .iter()
            .map(|value| {
                Table(value.as_table().unwrap_or_else(|| {
                    panic!("Table should have table array values for `{key}` key")
                }))
            })
            .collect()
    }

    pub fn get_command_id(&self, key: &str) -> CommandId {
        CommandId::new(self.get_uuid(key)).unwrap()
    }

    pub fn get_date_time(&self, key: &str) -> DateTime<Utc> {
        self.maybe_get_date_time(key)
            .unwrap_or_else(|| panic!("Table should have date time value for `{key}` key"))
    }

    pub fn get_integer(&self, key: &str) -> i64 {
        self.maybe_get_integer(key)
            .unwrap_or_else(|| panic!("Table should have integer value for `{key}` key"))
    }

    pub fn get_table(&self, key: &str) -> Table {
        self.maybe_get_table(key)
            .unwrap_or_else(|| panic!("Table should have table value for `{key}` key"))
    }

    pub fn get_text(&self, key: &str) -> &str {
        self.maybe_get_text(key).unwrap()
    }

    pub fn get_uuid(&self, key: &str) -> Uuid {
        self.maybe_get_uuid(key)
            .unwrap_or_else(|| panic!("Table should have uuid value for `{key}` key"))
    }

    pub fn get_workspace_id(&self, key: &str) -> WorkspaceId {
        self.maybe_get_workspace_id(key).unwrap()
    }

    pub fn maybe_get_date_time(&self, key: &str) -> Option<DateTime<Utc>> {
        self.maybe_get_text(key)
            .map(|value| NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S"))
            .transpose()
            .unwrap_or_else(|_| {
                panic!("Should be able to parse table key `{key}` as date time value")
            })
            .map(|date_time| date_time.and_utc())
    }

    pub fn maybe_get_integer(&self, key: &str) -> Option<i64> {
        self.0.get(key).and_then(|value| value.as_integer())
    }

    pub fn maybe_get_table(&self, key: &str) -> Option<Table> {
        self.0
            .get(key)
            .and_then(|value| value.as_table())
            .map(Table)
    }

    pub fn maybe_get_text(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.as_str())
    }

    pub fn maybe_get_uuid(&self, key: &str) -> Option<Uuid> {
        self.maybe_get_text(key).map(|text| {
            text.parse().unwrap_or_else(|_| {
                panic!("Should be able to parse table key `{key}` as uuid value")
            })
        })
    }

    pub fn maybe_get_workspace_id(&self, key: &str) -> Option<WorkspaceId> {
        self.maybe_get_uuid(key)
            .map(|uuid| WorkspaceId::new(uuid).unwrap())
    }
}
