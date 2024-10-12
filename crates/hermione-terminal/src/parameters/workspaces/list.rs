use crate::routes;

pub const PAGE_SIZE: u32 = 100;

pub struct Parameters {
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            page_number: 0,
            page_size: PAGE_SIZE,
        }
    }
}

impl From<Parameters> for routes::Route {
    fn from(value: Parameters) -> Self {
        Self::Workspaces(routes::workspaces::Route::List(value))
    }
}
