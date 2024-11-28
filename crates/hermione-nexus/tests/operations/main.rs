pub mod support;

mod workspaces;

mod copy_command_to_clipboard_test;
mod create_command_test;
// mod delete_backup_credentials_test;
// mod execute_program_test;
mod export_command_test;
mod export_workspace_test;
// mod import_commands_from_notion_test;
// mod import_workspaces_from_notion_test;
mod delete_command_test;
mod delete_commands_test;
mod export_commands_test;
mod export_workspaces_test;
mod save_backup_credentials_test;
// mod execute_command_test;
// mod export_test;
// mod get_command_test;
// mod import_test;
// mod list_backup_credentials_test;
// mod list_commands_test;
// mod update_command_test;

pub mod prelude {
    pub use toml::{toml as table, Table};

    use chrono::{DateTime, NaiveDateTime, Utc};
    use hermione_nexus::Error;
    use uuid::Uuid;

    pub struct Operation<T> {
        result: Option<Result<T, Error>>,
    }

    pub trait GetArray {
        fn get_array(&self, key: &str) -> Vec<&Table>;
    }

    pub trait GetDateTime {
        fn get_date_time(&self, key: &str) -> DateTime<Utc>;
    }

    pub trait GetInteger {
        fn get_integer(&self, key: &str) -> i64;
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

    pub trait MaybeGetInteger {
        fn maybe_get_integer(&self, key: &str) -> Option<i64>;
    }

    pub trait MaybeGetTable {
        fn maybe_get_table(&self, key: &str) -> Option<&Table>;
    }

    pub trait MaybeGetText {
        fn maybe_get_text(&self, key: &str) -> Option<&str>;
    }

    pub trait MaybeGetDateTime {
        fn maybe_get_date_time(&self, key: &str) -> Option<DateTime<Utc>>;
    }

    pub trait TestCase {
        fn execute_operation(&mut self, parameters: Table);
        fn inspect_operation_results(&self, expectations: Table);
        fn setup(&mut self, _parameters: Table) {}
    }

    impl<T> Operation<T> {
        pub fn assert_is_success(&self) {
            match self.result() {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            }
        }

        pub fn assert_is_failure(&self) {
            assert!(self.result().is_err());
        }

        pub fn result(&self) -> &Result<T, Error> {
            self.result
                .as_ref()
                .expect("Operation result should be present")
        }

        pub fn set_result(&mut self, result: Result<T, Error>) {
            self.result = Some(result);
        }
    }

    impl GetArray for Table {
        fn get_array(&self, key: &str) -> Vec<&Table> {
            let array_of_values = self
                .get(key)
                .and_then(|value| value.as_array())
                .unwrap_or_else(|| panic!("Table should have array value for `{key}` key"));

            array_of_values
                .iter()
                .map(|value| {
                    value.as_table().unwrap_or_else(|| {
                        panic!("Table should have table array values for `{key}` key")
                    })
                })
                .collect()
        }
    }

    impl GetDateTime for Table {
        fn get_date_time(&self, key: &str) -> DateTime<Utc> {
            self.maybe_get_date_time(key)
                .unwrap_or_else(|| panic!("Table should have date time value for `{key}` key"))
        }
    }

    impl GetInteger for Table {
        fn get_integer(&self, key: &str) -> i64 {
            self.maybe_get_integer(key)
                .unwrap_or_else(|| panic!("Table should have integer value for `{key}` key"))
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
            self.maybe_get_table(key)
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

    impl MaybeGetInteger for Table {
        fn maybe_get_integer(&self, key: &str) -> Option<i64> {
            self.get(key).and_then(|value| value.as_integer())
        }
    }

    impl MaybeGetTable for Table {
        fn maybe_get_table(&self, key: &str) -> Option<&Table> {
            self.get(key).and_then(|value| value.as_table())
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
