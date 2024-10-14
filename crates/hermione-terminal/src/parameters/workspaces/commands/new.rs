use crate::routes;

pub struct Parameters {
    pub workspace_id: String,
}

impl From<Parameters> for routes::Route {
    fn from(parameters: Parameters) -> Self {
        Self::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::New(parameters),
        ))
    }
}
