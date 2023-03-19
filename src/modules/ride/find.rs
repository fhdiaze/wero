use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::domain::location::Location;
use crate::domain::ride::Ride;
use crate::domain::route::Route;
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

pub async fn handle(db: DynDbClient, query: Query) -> Result<Page<RideVm>> {
  let rides = find_rides(db, query).await?;
  let rides_vm: Vec<RideVm> =
    rides.into_iter().map(|r| RideVm::from(&r)).collect();
  let page_size = rides_vm.len();

  Ok(Page::new(rides_vm, 1, page_size, 200))
}

async fn find_rides(db: DynDbClient, query: Query) -> Result<Vec<Ride>> {
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
  let rides: Vec<Ride> =
    db.rides().find(filter, None).await?.try_collect().await?;

  Ok(rides)
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
pub struct RouteVm {
  distance: f64,
  elevation: i32,
  profile: String,
  description: String,
}

impl RouteVm {
  fn from(route: &Route) -> Self {
    RouteVm {
      distance: route.distance,
      elevation: route.elevation,
      profile: route.profile.clone(),
      description: route.description.clone(),
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RideVm {
  id: String,
  name: String,
  description: String,
  start_at: DateTime<Utc>,
  discipline: String,
  category: String,
  route: RouteVm,
  location: LocationVm,
  website: String,
}

impl RideVm {
  fn from(ride: &Ride) -> Self {
    RideVm {
      id: ride.id.as_ref().unwrap().clone(),
      name: ride.name.clone(),
      description: ride.description.clone(),
      start_at: ride.start_at,
      discipline: ride.discipline.to_string(),
      category: ride.category.to_string(),
      route: RouteVm::from(&ride.route),
      location: LocationVm::from(&ride.location),
      website: ride.website.clone(),
    }
  }
}
