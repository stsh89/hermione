use crate::routes;

pub struct Parameters {
    pub command_id: String,
    pub workspace_id: String,
}

impl From<Parameters> for routes::Route {
    fn from(parameters: Parameters) -> Self {
        Self::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::Edit(parameters),
        ))
    }
}
