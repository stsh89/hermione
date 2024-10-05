pub mod rich_text;
pub mod title;
pub mod unique_id;

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::OptionExt;
    use serde::Deserialize;
    use serde_json::Value;

    type Result<T> = eyre::Result<T>;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Name"), deserialize_with = "title::deserializer")]
        name: Option<String>,

        #[serde(
            rename(deserialize = "ID"),
            deserialize_with = "unique_id::deserializer"
        )]
        id: u64,

        #[serde(
            rename(deserialize = "External ID"),
            deserialize_with = "rich_text::deserializer"
        )]
        external_id: Option<String>,

        #[serde(
            rename(deserialize = "Last access time"),
            deserialize_with = "rich_text::deserializer"
        )]
        last_access_time: Option<String>,

        #[serde(
            rename(deserialize = "Location"),
            deserialize_with = "rich_text::deserializer"
        )]
        location: Option<String>,
    }

    fn json() -> String {
        r#"{
            "Name": {
                "id": "7DUIF",
                "type": "title",
                "title": []
            },
            "ID": {
                "id": "jZ%3DO",
                "type": "unique_id",
                "unique_id": {
                    "number": 4,
                    "prefix": null
                }
            },
            "External ID": {
                "id": "k5DnL",
                "type": "rich_text",
                "rich_text": []
            },
            "Last access time": {
                "id": "k5DnL",
                "type": "rich_text",
                "rich_text": []
            },
            "Location": {
                "id": "k5DnL",
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
                        "plain_text": "C:\\",
                        "text": {
                            "content": "C:\\",
                            "link": null
                        },
                        "type": "text"
                    }
                ]
            }
        }"#
        .to_string()
    }

    #[test]
    fn test_deserializer_from_string() -> Result<()> {
        let record: Record = serde_json::from_str(json().as_str())?;

        assert_eq!(record.name, None);
        assert_eq!(record.id, 4);
        assert_eq!(record.external_id, None);
        assert_eq!(record.last_access_time, None);
        assert_eq!(record.location, Some("C:\\".to_string()));

        Ok(())
    }

    #[test]
    fn test_deserializer_from_value() -> Result<()> {
        let value: Value = serde_json::from_str(json().as_str())?;

        let record: Record = serde_json::from_value(value)?;

        assert_eq!(record.name, None);
        assert_eq!(record.id, 4);
        assert_eq!(record.external_id, None);
        assert_eq!(record.last_access_time, None);
        assert_eq!(record.location, Some("C:\\".to_string()));

        Ok(())
    }

    #[test]
    fn test_deserializer_from_values() -> Result<()> {
        let value: Value = serde_json::from_str(json().as_str())?;
        let values = vec![value];

        let records = values
            .into_iter()
            .map(|r| Ok(serde_json::from_value(r)?))
            .collect::<Result<Vec<Record>>>()?;

        let record = records
            .into_iter()
            .next()
            .ok_or_eyre("Expected one record")?;

        assert_eq!(record.name, None);
        assert_eq!(record.id, 4);
        assert_eq!(record.external_id, None);
        assert_eq!(record.last_access_time, None);
        assert_eq!(record.location, Some("C:\\".to_string()));

        Ok(())
    }
}
