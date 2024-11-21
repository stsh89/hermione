pub mod file_system;
pub mod notion;
pub mod powershell;
pub mod sqlite;

const APPLICATION_STATE: ApplicationState = ApplicationState::evaluate();

enum ApplicationState {
    Release,
    Development,
}

impl ApplicationState {
    const fn evaluate() -> Self {
        if cfg!(not(debug_assertions)) {
            return Self::Release;
        }

        Self::Development
    }
}
