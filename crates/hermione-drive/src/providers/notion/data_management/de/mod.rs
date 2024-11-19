pub mod rich_text;
pub mod title;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    struct QueryDatabaseResponse<T> {
        #[serde(rename(deserialize = "results"))]
        database_pages: Vec<DatabasePage<T>>,

        next_cursor: Option<String>,
    }

    #[derive(Deserialize)]
    struct DatabasePage<T> {
        #[serde(rename(deserialize = "id"))]
        page_id: String,

        properties: T,
    }

    #[derive(Deserialize)]
    struct Product {
        #[serde(rename(deserialize = "Name"), deserialize_with = "title::deserializer")]
        name: String,

        #[serde(
            rename(deserialize = "Description"),
            deserialize_with = "rich_text::deserializer"
        )]
        description: String,
    }

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "Name"), deserialize_with = "title::deserializer")]
        name: String,

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

    fn query_database_response_json() -> String {
        r#"
{
  "object": "list",
  "results": [
    {
      "object": "page",
      "id": "59833787-2cf9-4fdf-8782-e53db20768a5",
      "created_time": "2022-03-01T19:05:00.000Z",
      "last_edited_time": "2022-07-06T20:25:00.000Z",
      "created_by": {
        "object": "user",
        "id": "ee5f0f84-409a-440f-983a-a5315961c6e4"
      },
      "last_edited_by": {
        "object": "user",
        "id": "0c3e9826-b8f7-4f73-927d-2caaf86f1103"
      },
      "cover": {
        "type": "external",
        "external": {
          "url": "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg"
        }
      },
      "icon": {
        "type": "emoji",
        "emoji": "🥬"
      },
      "parent": {
        "type": "database_id",
        "database_id": "d9824bdc-8445-4327-be8b-5b47500af6ce"
      },
      "archived": false,
      "properties": {
        "Description": {
          "id": "_Tc_",
          "type": "rich_text",
          "rich_text": [
            {
              "type": "text",
              "text": {
                "content": "A dark ",
                "link": null
              },
              "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "default"
              },
              "plain_text": "A dark ",
              "href": null
            },
            {
              "type": "text",
              "text": {
                "content": "green",
                "link": null
              },
              "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "green"
              },
              "plain_text": "green",
              "href": null
            },
            {
              "type": "text",
              "text": {
                "content": " leafy vegetable",
                "link": null
              },
              "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "default"
              },
              "plain_text": " leafy vegetable",
              "href": null
            }
          ]
        },
        "Number of meals": {
          "id": "zag~",
          "type": "rollup",
          "rollup": {
            "type": "number",
            "number": 2,
            "function": "count"
          }
        },
        "Photo": {
          "id": "%7DF_L",
          "type": "url",
          "url": "https://i.insider.com/612fb23c9ef1e50018f93198?width=1136&format=jpeg"
        },
        "Name": {
          "id": "title",
          "type": "title",
          "title": [
            {
              "type": "text",
              "text": {
                "content": "Tuscan kale",
                "link": null
              },
              "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "default"
              },
              "plain_text": "Tuscan kale",
              "href": null
            }
          ]
        }
      },
      "url": "https://www.notion.so/Tuscan-kale-598337872cf94fdf8782e53db20768a5"
    }
  ],
  "next_cursor": null,
  "has_more": false,
  "type": "page_or_database",
  "page_or_database": {}
}
        "#
        .to_string()
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
        let record: Record = serde_json::from_str(&json())?;

        assert_eq!(record.name, "");
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
            .map(serde_json::from_value)
            .collect::<Result<Vec<Record>, serde_json::Error>>()?;

        assert_eq!(records.len(), 1);

        let record = &records[0];

        assert_eq!(record.name, "");
        assert_eq!(record.external_id, "");
        assert_eq!(record.last_access_time, "");
        assert_eq!(record.location, "C:\\");

        Ok(())
    }

    #[test]
    fn test_deserialize_query_database_response() -> Result<(), serde_json::Error> {
        let json_string = query_database_response_json();
        let mut query_database_response: QueryDatabaseResponse<Product> =
            serde_json::from_str(&json_string)?;

        assert_eq!(query_database_response.database_pages.len(), 1);
        assert_eq!(query_database_response.next_cursor, None);

        let page = query_database_response.database_pages.pop().unwrap();

        assert_eq!(page.page_id, "59833787-2cf9-4fdf-8782-e53db20768a5");
        assert_eq!(page.properties.name, "Tuscan kale");
        assert_eq!(page.properties.description, "A dark green leafy vegetable");

        Ok(())
    }
}
