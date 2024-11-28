pub mod support;

mod commands;
mod workspaces;

// mod delete_backup_credentials_test;
mod export_command_test;
mod export_workspace_test;
// mod import_commands_from_notion_test;
// mod import_workspaces_from_notion_test;
mod export_commands_test;
mod export_workspaces_test;
mod save_backup_credentials_test;
// mod execute_command_test;
// mod export_test;
// mod import_test;
// mod list_backup_credentials_test;

pub mod prelude {
    pub use crate::support::table::*;

    use hermione_nexus::Error;

    pub struct Operation<T> {
        result: Option<Result<T, Error>>,
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

    impl<T> Default for Operation<T> {
        fn default() -> Self {
            Self { result: None }
        }
    }
}
