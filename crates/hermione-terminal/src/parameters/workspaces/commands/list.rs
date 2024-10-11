pub const PAGE_SIZE: u32 = 100;

pub struct Parameters {
    pub workspace_id: String,
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}
