pub mod rich_text;
pub mod title;
pub mod unique_id;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Name"), deserialize_with = "title::deserializer")]
        name: String,

        #[serde(
            rename(deserialize = "ID"),
            deserialize_with = "unique_id::deserializer"
        )]
        id: u64,

        #[serde(
            rename(deserialize = "External ID"),
            deserialize_with = "rich_text::deserializer"
        )]
        external_id: String,

        #[serde(
            rename(deserialize = "Last access time"),
            deserialize_with = "rich_text::deserializer"
        )]
        last_access_time: String,

        #[serde(
            rename(deserialize = "Location"),
            deserialize_with = "rich_text::deserializer"
        )]
        location: String,
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
    fn test_deserializer_from_string() -> Result<(), serde_json::Error> {
        let record: Record = serde_json::from_str(json().as_str())?;

        assert_eq!(record.name, "");
        assert_eq!(record.id, 4);
        assert_eq!(record.external_id, "");
        assert_eq!(record.last_access_time, "");
        assert_eq!(record.location, "C:\\");

        Ok(())
    }

    #[test]
    fn test_deserializer_from_value() -> Result<(), serde_json::Error> {
        let value: Value = serde_json::from_str(json().as_str())?;

        let record: Record = serde_json::from_value(value)?;

        assert_eq!(record.name, "");
        assert_eq!(record.id, 4);
        assert_eq!(record.external_id, "");
        assert_eq!(record.last_access_time, "");
        assert_eq!(record.location, "C:\\");

        Ok(())
    }

    #[test]
    fn test_deserializer_from_values() -> Result<(), serde_json::Error> {
        let value: Value = serde_json::from_str(json().as_str())?;
        let values = vec![value];

        let records = values
            .into_iter()
            .map(|r| Ok(serde_json::from_value(r)?))
            .collect::<Result<Vec<Record>, serde_json::Error>>()?;

        assert_eq!(records.len(), 1);

        let record = &records[0];

        assert_eq!(record.name, "");
        assert_eq!(record.id, 4);
        assert_eq!(record.external_id, "");
        assert_eq!(record.last_access_time, "");
        assert_eq!(record.location, "C:\\");

        Ok(())
    }
}
