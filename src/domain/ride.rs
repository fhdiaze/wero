use super::{discipline::Discipline, location::Location, route::Route};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ride {
  #[serde(
    rename = "_id",
    with = "hex_string_as_object_id",
    skip_serializing_if = "Option::is_none"
  )]
  pub id: Option<String>,
  pub name: String,
  pub description: String,
  pub start_at: DateTime<Utc>,
  pub route: Route,
  pub discipline: Discipline,
  pub location: Location,
  pub image: String,
}

impl Ride {
  /// Creates a ride
  pub fn new(
    id: Option<String>,
    name: String,
    description: String,
    start_at: DateTime<Utc>,
    route: Route,
    discipline: Discipline,
    location: Location,
    image: String,
  ) -> Self {
    Ride {
      id,
      name,
      description,
      start_at,
      route,
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
  pub fn deserialize<'de, D>(
    deserializer: D,
  ) -> Result<Option<String>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let object_id = ObjectId::deserialize(deserializer)?;
    Ok(Some(object_id.to_hex()))
  }

  /// Serializes a hex string as an ObjectId.
  pub fn serialize<S>(
    val: &Option<String>,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
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
