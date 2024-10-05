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
        let mut id: Option<u64> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "id" | "type" => {
                    map.next_value::<String>()?;
                }
                "unique_id" => {
                    id = Some(get_id(&mut map)?);
                }
                _ => return Err(Error::unknown_field(&key, &["id", "type", "unique_id"])),
            }
        }

        id.ok_or(Error::missing_field("unique_id"))
    }
}

fn get_id<'de, V>(map: &mut V) -> Result<u64, V::Error>
where
    V: MapAccess<'de>,
{
    let unique_id = map.next_value::<UniqueId>()?;

    Ok(unique_id.number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename(deserialize = "ID"), deserialize_with = "deserializer")]
        id: u64,
    }

    #[test]
    fn test_deserializer() -> Result<(), serde_json::Error> {
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
