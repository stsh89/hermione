use ratatui::text::Text;

pub mod default;
pub mod search_list;

#[derive(Default)]
pub struct Breadcrumbs {
    segments: Vec<String>,
}

impl Breadcrumbs {
    pub fn add_segment(mut self, segment: impl ToString) -> Self {
        self.segments.push(segment.to_string());
        self
    }
}

impl std::fmt::Display for Breadcrumbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.segments.join(" > "))
    }
}

impl<'a> From<Breadcrumbs> for Text<'a> {
    fn from(breadcrumb: Breadcrumbs) -> Self {
        Text::from(breadcrumb.to_string())
    }
}
