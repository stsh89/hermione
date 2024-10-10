pub const PAGE_SIZE: u32 = 1;

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
