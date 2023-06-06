use crate::{
  domain::{contact::Contact, details::Details, ride::Ride},
  infra::{core::result::Result, db::traits::DynDbClient},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Command {
  pub name: String,
  pub description: String,
  pub start_at: DateTime<Utc>,
  pub details: Details,
  pub contact: Contact,
}

pub async fn handle(db: DynDbClient, cmd: Command) -> Result<RideVm> {
  let ride = Ride::new(
    None,
    cmd.name,
    cmd.description,
    cmd.start_at,
    cmd.details,
    cmd.contact,
  )?;
  let result = db.rides().insert_one(ride, None).await?;
  let ride_id = result.inserted_id.to_string();

  Ok(RideVm::new(ride_id))
}

#[derive(Serialize)]
pub struct RideVm {
  id: String,
}

impl RideVm {
  fn new(id: String) -> Self {
    RideVm { id }
  }
}
