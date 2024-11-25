pub mod support;

mod copy_command_to_clipboard_test;
mod create_command_test;
mod create_workspace_test;
// mod delete_backup_credentials_test;
// mod execute_program_test;
mod export_workspace_test;
// mod import_commands_from_notion_test;
// mod import_workspaces_from_notion_test;
mod save_backup_credentials_test;
mod visit_workspace_location_test;
// mod delete_workspace_test;
// mod detele_command_test;
// mod execute_command_test;
// mod export_test;
// mod get_command_test;
// mod get_workspace_test;
// mod import_test;
// mod list_backup_credentials_test;
// mod list_commands_test;
// mod list_workspaces_test;
// mod update_command_test;
// mod update_workspace_test;

pub mod prelude {
    pub use toml::{toml as table, Table};

    use chrono::{DateTime, NaiveDateTime, Utc};
    use uuid::Uuid;

    pub struct Operation<T> {
        result: Option<hermione_nexus::Result<T>>,
    }

    pub trait GetDateTime {
        fn get_date_time(&self, key: &str) -> DateTime<Utc>;
    }

    pub trait GetText {
        fn get_text(&self, key: &str) -> &str;
    }

    pub trait GetTable {
        fn get_table(&self, key: &str) -> &Table;
    }

    pub trait GetUuid {
        fn get_uuid(&self, key: &str) -> Uuid;
    }

    pub trait MaybeGetText {
        fn maybe_get_text(&self, key: &str) -> Option<&str>;
    }

    pub trait MaybeGetDateTime {
        fn maybe_get_date_time(&self, key: &str) -> Option<DateTime<Utc>>;
    }

    pub trait TestCase {
        fn assert_operation_failure(&self, _parameters: Table) {}
        fn assert_operation_success(&self, parameters: Table);
        fn execute_operation(&mut self, parameters: Table);
        fn setup(&mut self, _parameters: Table) {}
    }

    impl<T> Operation<T> {
        pub fn assert_success(&self) {
            match self.result() {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            }
        }

        pub fn assert_failure(&self) {
            assert!(self.result().is_err(), "Expected operation to fail");
        }

        pub fn result(&self) -> &hermione_nexus::Result<T> {
            self.result
                .as_ref()
                .expect("Operation result should be present")
        }

        pub fn set_result(&mut self, result: hermione_nexus::Result<T>) {
            self.result = Some(result);
        }
    }

    impl GetDateTime for Table {
        fn get_date_time(&self, key: &str) -> DateTime<Utc> {
            self.maybe_get_date_time(key)
                .unwrap_or_else(|| panic!("Table should have date time value for `{key}` key"))
        }
    }

    impl GetText for Table {
        fn get_text(&self, key: &str) -> &str {
            self.maybe_get_text(key)
                .unwrap_or_else(|| panic!("Table should have text value for `{key}` key"))
        }
    }

    impl GetTable for Table {
        fn get_table(&self, key: &str) -> &Table {
            self.get(key)
                .and_then(|value| value.as_table())
                .unwrap_or_else(|| panic!("Table should have table value for `{key}` key"))
        }
    }

    impl GetUuid for Table {
        fn get_uuid(&self, key: &str) -> Uuid {
            self.get_text(key).parse().unwrap_or_else(|_| {
                panic!("Should be able to parse table key `{key}` as uuid value")
            })
        }
    }

    impl MaybeGetText for Table {
        fn maybe_get_text(&self, key: &str) -> Option<&str> {
            self.get(key).and_then(|value| value.as_str())
        }
    }

    impl MaybeGetDateTime for Table {
        fn maybe_get_date_time(&self, key: &str) -> Option<DateTime<Utc>> {
            self.maybe_get_text(key)
                .map(|value| NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S"))
                .transpose()
                .unwrap_or_else(|_| {
                    panic!("Should be able to parse table key `{key}` as date time value")
                })
                .map(|date_time| date_time.and_utc())
        }
    }

    impl<T> Default for Operation<T> {
        fn default() -> Self {
            Self { result: None }
        }
    }
}
