use crate::routes;

pub struct Parameters {
    pub name: String,
    pub location: String,
}

impl From<Parameters> for routes::Route {
    fn from(value: Parameters) -> Self {
        Self::Workspaces(routes::workspaces::Route::Create(value))
    }
}
