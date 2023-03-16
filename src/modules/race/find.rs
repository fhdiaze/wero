use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::domain::location::Location;
use crate::domain::race::Race;
use crate::infra::{
  core::{page::Page, result::Result},
  db::traits::DynDbClient,
};

#[derive(Deserialize)]
pub struct Query {
  pub name: Option<String>,
  pub city: Option<String>,
  pub country: Option<String>,
}

impl Query {
  pub fn new(
    name: Option<String>,
    city: Option<String>,
    country: Option<String>,
  ) -> Self {
    Query {
      name,
      city,
      country,
    }
  }
}

pub async fn handle(db: DynDbClient, query: Query) -> Result<Page<RaceVm>> {
  let races = find_races(db, query).await?;
  let races_vm: Vec<RaceVm> =
    races.into_iter().map(|r| RaceVm::from(&r)).collect();
  let page_size = races_vm.len();

  Ok(Page::new(races_vm, 1, page_size, 200))
}

async fn find_races(db: DynDbClient, query: Query) -> Result<Vec<Race>> {
  let mut filter = doc! {};

  if query.name.is_some() {
    filter.insert("name", query.name.unwrap());
  }

  if query.city.is_some() {
    filter.insert("location.city", query.city.unwrap());
  }

  if query.country.is_some() {
    filter.insert("location.country", query.country.unwrap());
  }
  let races: Vec<Race> =
    db.races().find(filter, None).await?.try_collect().await?;

  Ok(races)
}

#[derive(Serialize)]
pub struct LocationVm {
  address: String,
  city: String,
  region: String,
  country: String,
}

impl LocationVm {
  fn from(location: &Location) -> Self {
    LocationVm {
      address: location.address.clone(),
      city: location.city.clone(),
      region: location.region.clone(),
      country: location.country.clone(),
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceVm {
  id: String,
  name: String,
  description: String,
  start_at: DateTime<Utc>,
  distance: f64,
  location: LocationVm,
}

impl RaceVm {
  fn from(race: &Race) -> Self {
    RaceVm {
      id: race.id.as_ref().unwrap().clone(),
      name: race.name.clone(),
      description: race.description.clone(),
      start_at: race.start_at,
      distance: race.route.distance,
      location: LocationVm::from(&race.location),
    }
  }
}
