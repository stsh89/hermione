use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

pub fn deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
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
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with id, type, and rich_text fields")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut rich_text: Option<String> = None;

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

fn get_rich_text<'de, V>(map: &mut V) -> Result<String, V::Error>
where
    V: MapAccess<'de>,
{
    let text = map
        .next_value::<Vec<RichText>>()?
        .into_iter()
        .map(|r| r.plain_text)
        .collect::<Vec<String>>()
        .join("");

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Description"), deserialize_with = "deserializer")]
        description: String,
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

        assert_eq!(record.description, "");

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

        assert_eq!(record.description, "Test description");

        Ok(())
    }
}
