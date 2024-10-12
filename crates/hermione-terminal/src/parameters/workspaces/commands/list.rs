use crate::routes;

pub const PAGE_SIZE: u32 = 100;

pub struct Parameters {
    pub workspace_id: String,
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}

impl From<Parameters> for routes::Route {
    fn from(value: Parameters) -> Self {
        Self::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(value),
        ))
    }
}
