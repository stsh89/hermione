use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

pub fn deserializer<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(UniqueIdVisitor)
}

struct UniqueIdVisitor;

#[derive(Debug, Deserialize)]
struct UniqueId {
    number: u64,
}

impl<'de> Visitor<'de> for UniqueIdVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with nested unique_id structure")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<u64, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut id = 0;
        let mut key_found = false;

        while let Some(key) = map.next_key::<String>()? {
            if key == "unique_id" {
                key_found = true;
                let unique_id: UniqueId = map.next_value()?;
                id = unique_id.number;
            } else {
                let _ = map.next_value::<serde_json::Value>()?;
            }
        }

        if !key_found {
            return Err(Error::missing_field("unique_id"));
        }

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "ID"), deserialize_with = "deserializer")]
        id: u64,
    }

    #[test]
    fn test_deserializer() -> Result<()> {
        let json = r#"{
            "ID": {
                "id": "jZ%3DO",
                "type": "unique_id",
                "unique_id": {
                    "number": 4,
                    "prefix": null
                }
            }
        }"#;

        let record: Record = serde_json::from_str(json)?;

        assert_eq!(record.id, 4);

        Ok(())
    }
}
