use crate::{
  domain::{
    contact::Contact, discipline::Discipline, format::Format, ride::Ride,
    route::Route,
  },
  infra::{core::result::AppResult, db::traits::DynDbClient},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Command {
  pub name: String,
  pub description: String,
  pub start_at: DateTime<Utc>,
  pub route: Route,
  pub discipline: Discipline,
  pub format: Format,
  pub contact: Contact,
}

pub async fn handle(db: DynDbClient, cmd: Command) -> AppResult<RideVm> {
  let ride = cmd.to_ride()?;
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

impl Command {
  fn to_ride(&self) -> AppResult<Ride> {
    Ride::new(
      None,
      self.name.clone(),
      self.description.clone(),
      self.start_at,
      self.route.clone(),
      self.discipline.clone(),
      self.format.clone(),
      self.contact.clone(),
    )
  }
}
