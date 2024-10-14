use crate::routes;

pub struct Parameters {
    pub id: String,
}

impl From<Parameters> for routes::Route {
    fn from(value: Parameters) -> Self {
        Self::Workspaces(routes::workspaces::Route::Edit(value))
    }
}
