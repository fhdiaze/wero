use serde::{Deserialize, Serialize};

use super::{discipline::Discipline, location::Location};

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    #[serde(
        rename = "_id",
        with = "hex_string_as_object_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    pub name: String,
    pub distance: f64,
    pub discipline: Discipline,
    pub location: Location,
    pub image: String,
}

impl Race {
    /// Creates a race
    pub fn new(
        id: Option<String>,
        name: String,
        distance: f64,
        discipline: Discipline,
        location: Location,
        image: String,
    ) -> Self {
        Race {
            id,
            name,
            distance,
            discipline,
            location,
            image,
        }
    }
}

pub mod hex_string_as_object_id {
    use bson::oid::ObjectId;
    use serde::{ser, Deserialize, Deserializer, Serialize, Serializer};

    /// Deserializes a hex string from an ObjectId.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let object_id = ObjectId::deserialize(deserializer)?;
        Ok(Some(object_id.to_hex()))
    }

    /// Serializes a hex string as an ObjectId.
    pub fn serialize<S>(val: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match ObjectId::parse_str(val.as_ref().unwrap()) {
            Ok(oid) => oid.serialize(serializer),
            Err(_) => Err(ser::Error::custom(format!(
                "cannot convert {} to ObjectId",
                val.as_ref().unwrap()
            ))),
        }
    }
}
