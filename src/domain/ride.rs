use super::{
  format::Format, discipline::Discipline, location::Location, route::Route,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ride {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: String,
  pub description: String,
  pub start_at: DateTime<Utc>,
  pub route: Route,
  pub discipline: Discipline,
  pub format: Format,
  pub location: Location,
  pub website: String,
}

impl Ride {
  /// Creates a ride
  pub fn new(
    id: Option<ObjectId>,
    name: String,
    description: String,
    start_at: DateTime<Utc>,
    route: Route,
    discipline: Discipline,
    format: Format,
    location: Location,
    website: String,
  ) -> Self {
    Ride {
      id,
      name,
      description,
      start_at,
      route,
      discipline,
      format,
      location,
      website,
    }
  }
}
