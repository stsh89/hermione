use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

pub fn deserializer<'de, D>(deserializer: D) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(RichTextVisitor)
}

struct RichTextVisitor;

#[derive(Debug, Deserialize)]
struct RichText {
    plain_text: String,
}

impl<'de> Visitor<'de> for RichTextVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with nested rich_text structure")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<Option<String>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut rich_text: Option<Option<String>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "id" | "type" => {
                    map.next_value::<String>()?;
                }
                "rich_text" => {
                    rich_text = Some(get_rich_text(&mut map)?);
                }
                _ => return Err(Error::unknown_field(&key, &["id", "type", "rich_text"])),
            }
        }

        rich_text.ok_or(Error::missing_field("rich_text"))
    }
}

fn get_rich_text<'de, V>(map: &mut V) -> Result<Option<String>, V::Error>
where
    V: MapAccess<'de>,
{
    let mut rich_text = map.next_value::<Vec<RichText>>()?;

    if rich_text.is_empty() {
        return Ok(None);
    }

    let plain_text = rich_text.remove(0).plain_text;

    Ok(Some(plain_text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Description"), deserialize_with = "deserializer")]
        description: Option<String>,
    }

    #[test]
    fn test_deserializer_if_empty_array() -> Result<(), serde_json::Error> {
        let json = r#"{
            "Description": {
                "id": "7DUIF",
                "type": "rich_text",
                "rich_text": []
            }
        }"#;

        let record: Record = serde_json::from_str(json)?;

        assert_eq!(record.description, None);

        Ok(())
    }

    #[test]
    fn test_deserializer() -> Result<(), serde_json::Error> {
        let json = r#"{
            "Description": {
                "id": "7DUIF",
                "type": "rich_text",
                "rich_text": [
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
                        "plain_text": "Test description",
                        "text": {
                            "content": "Test description",
                            "link": null
                        },
                        "type": "text"
                    }
                ]
            }
        }"#;

        let record: Record = serde_json::from_str(json)?;

        assert_eq!(record.description, Some("Test description".into()));

        Ok(())
    }
}
