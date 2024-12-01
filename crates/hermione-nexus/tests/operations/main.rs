#[macro_use]
pub mod support;

mod backup;
mod commands;

mod create_command;
mod create_workspace;
mod delete_command;
mod delete_commands;
mod delete_workspace;
mod get_command;
mod get_workspace;
mod list_workspaces;
mod update_workspace;
mod visit_workspace_location;

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
