use crate::domain::location::Location;
use crate::domain::ride::Ride;
use crate::domain::route::Route;
use crate::infra::core::paging::Cursor;
use crate::infra::{
  core::{paging::Page, result::Result},
  db::traits::DynDbClient,
};
use bson::Document;
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
  pub name: Option<String>,
  pub description: Option<String>,
  pub city: Option<String>,
  pub country: Option<String>,
}

pub async fn handle(
  db: DynDbClient,
  cursor: Cursor<Query>,
) -> Result<Page<RideVm>> {
  let rides = find_rides(db, cursor).await?;
  let rides_vm: Vec<RideVm> =
    rides.into_iter().map(|r| RideVm::from(&r)).collect();
  let page_size = rides_vm.len();
  let continuation_token = rides_vm.last().map(|i| i.id.clone());

  Ok(Page::new(rides_vm, page_size, continuation_token))
}

async fn find_rides(
  db: DynDbClient,
  cursor: Cursor<Query>,
) -> Result<Vec<Ride>> {
  let options = FindOptions::builder()
    .sort(doc! {"_id": -1})
    .limit(cursor.size)
    .build();

  let filter = build_filter(cursor);

  let rides: Vec<Ride> = db
    .rides()
    .find(filter, options)
    .await?
    .try_collect()
    .await?;

  Ok(rides)
}

fn build_filter(cursor: Cursor<Query>) -> Document {
  let mut filter = doc! {};

  // Get just future rides
  filter.insert("start_at", doc! {"$gt": Utc::now().to_string()});

  if let Some(continuation_token) = cursor.continuation_token {
    filter.insert("_id", doc! { "$lt": continuation_token });
  }

  if let Some(query) = cursor.query {
    let mut conditions: Vec<Document> = vec![];

    if let Some(name) = query.name.filter(|x| !x.is_empty()) {
      conditions.push(doc! {"name": {"$regex": name, "$options": "i"}});
    }

    if let Some(city) = query.city.filter(|x| !x.is_empty()) {
      conditions.push(doc! { "location.city": {"$regex": city, "$options": "i"} });
    }

    if let Some(country) = query.country.filter(|x| !x.is_empty()) {
      conditions.push(doc! { "location.country": {"$regex": country, "$options": "i"} });
    }

    if !conditions.is_empty() {
      filter.insert("$or", conditions);
    }
  }

  tracing::info!("Query sent={}", filter);

  filter
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
      id: ride.id.unwrap().to_string(),
      name: ride.name.clone(),
      description: ride.description.clone(),
      start_at: ride.start_at,
      discipline: ride.discipline.to_string(),
      category: ride.format.to_string(),
      route: RouteVm::from(&ride.route),
      location: LocationVm::from(&ride.location),
      website: ride.website.clone(),
    }
  }
}
