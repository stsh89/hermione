use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

pub fn deserializer<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
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
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with id, type, and title fields")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut title: Option<String> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "id" | "type" => {
                    map.next_value::<String>()?;
                }
                "title" => {
                    title = Some(get_title(&mut map)?);
                }
                _ => return Err(Error::unknown_field(&key, &["id", "type", "title"])),
            }
        }

        title.ok_or(Error::missing_field("title"))
    }
}

fn get_title<'de, V>(map: &mut V) -> Result<String, V::Error>
where
    V: MapAccess<'de>,
{
    let text = map
        .next_value::<Vec<Title>>()?
        .into_iter()
        .map(|t| t.plain_text)
        .collect::<Vec<String>>()
        .join("");

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Name"), deserialize_with = "deserializer")]
        name: String,
    }

    #[test]
    fn test_deserializer_if_empty_array() -> Result<(), serde_json::Error> {
        let json = r#"{
            "Name": {
                "id": "7DUIF",
                "type": "title",
                "title": []
            }
        }"#;

        let record: Record = serde_json::from_str(json)?;

        assert_eq!(record.name, "");

        Ok(())
    }

    #[test]
    fn test_deserializer() -> Result<(), serde_json::Error> {
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

        assert_eq!(record.name, "Test title");

        Ok(())
    }
}
