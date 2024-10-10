pub type Json = serde_json::Value;

pub trait RichText {
    fn rich_text(&self, property_name: &str) -> &str;
}

pub trait Title {
    fn title(&self) -> &str;
}

pub trait PageId {
    fn id(&self) -> &str;
}

impl RichText for Json {
    fn rich_text(&self, property_name: &str) -> &str {
        self["properties"][property_name]["rich_text"][0]["plain_text"]
            .as_str()
            .unwrap_or_default()
    }
}

impl Title for Json {
    fn title(&self) -> &str {
        self["properties"]["Name"]["title"][0]["plain_text"]
            .as_str()
            .unwrap_or_default()
    }
}

impl PageId for Json {
    fn id(&self) -> &str {
        self["id"].as_str().unwrap_or_default()
    }
}
