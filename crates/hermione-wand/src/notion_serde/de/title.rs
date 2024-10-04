use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

pub fn deserializer<'de, D>(deserializer: D) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(TitleVisitor)
}

struct TitleVisitor;

#[derive(Debug, Deserialize)]
struct Title {
    plain_text: String,
}

impl<'de> Visitor<'de> for TitleVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with nested title structure")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<Option<String>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut option: Option<String> = None;
        let mut key_found = false;

        while let Some(key) = map.next_key::<String>()? {
            if key == "title" {
                key_found = true;
                let mut title = map.next_value::<Vec<Title>>()?;

                if !title.is_empty() {
                    let plain_text = title.remove(0).plain_text;

                    if !plain_text.is_empty() {
                        option = Some(plain_text);
                    }
                }
            } else {
                let _ = map.next_value::<serde_json::Value>()?;
            }
        }

        if !key_found {
            return Err(Error::missing_field("title"));
        }

        Ok(option)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Name"), deserialize_with = "deserializer")]
        name: Option<String>,
    }

    #[test]
    fn test_deserializer_if_empty_array() -> Result<()> {
        let json = r#"{
            "Name": {
                "id": "7DUIF",
                "type": "title",
                "title": []
            }
        }"#;

        let record: Record = serde_json::from_str(json)?;

        assert_eq!(record.name, None);

        Ok(())
    }

    #[test]
    fn test_deserializer() -> Result<()> {
        let json = r#"{
            "Name": {
                "id": "7DUIF",
                "type": "title",
                "title": [
                    {
                        "annotations": {
                            "bold": false,
                            "code": false,
                            "color": "default",
                            "italic": false,
                            "strikethrough": false,
                            "underline": false
                        },
                        "href": null,
                        "plain_text": "Test title",
                        "text": {
                            "content": "Test title",
                            "link": null
                        },
                        "type": "text"
                    }
                ]
            }
        }"#;

        let record: Record = serde_json::from_str(json)?;

        assert_eq!(record.name, Some("Test title".into()));

        Ok(())
    }
}
