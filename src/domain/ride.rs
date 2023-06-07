use super::{
  contact::Contact, discipline::Discipline, format::Format, route::Route,
};
use crate::infra::{core::result::Result, error::AppError};
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
  pub contact: Contact,
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
    contact: Contact,
  ) -> Result<Self> {
    if name.is_empty() {
      return Err(AppError::Validation(String::from(
        "The name of a ride could not be empty",
      )));
    }

    Ok(Self {
      id,
      name,
      description,
      start_at,
      route,
      discipline,
      format,
      contact,
    })
  }
}
