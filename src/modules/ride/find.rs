use crate::{
  domain::{contact::Contact, location::Location, ride::Ride, route::Route},
  infra::{
    core::{
      paging::{Cursor, Page},
      result::AppResult,
    },
    db::traits::DynDbClient,
  },
};
use bson::Document;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Handles a rides query
pub async fn handle(
  db: DynDbClient,
  cursor: Cursor<Query>,
) -> AppResult<Page<RideVm>> {
  let page = cursor.page_number;
  let rides_vm = find_rides(db, cursor).await.map(RideVm::from_many)?;

  Ok(Page::new(rides_vm, page))
}

async fn find_rides(
  db: DynDbClient,
  cursor: Cursor<Query>,
) -> AppResult<Vec<Ride>> {
  let options = build_options(cursor.page_number, cursor.page_size);
  let filter = cursor.query.map(to_filter);

  let rides: Vec<Ride> = db
    .rides()
    .find(filter, options)
    .await?
    .try_collect()
    .await?;

  Ok(rides)
}

fn build_options(page_number: usize, page_size: usize) -> FindOptions {
  let offset = page_number * page_size;
  FindOptions::builder()
    .sort(doc! { "start_at": 1, "_id": 1 })
    .skip(offset as u64)
    .limit(page_size as i64)
    .build()
}

fn to_filter(query: Query) -> Document {
  let mut filter = doc! {};

  // Get just future rides
  filter.insert("start_at", doc! {"$gt": Utc::now().to_string()});

  to_conditions(query).map(|x| filter.insert("$or", x));

  info!("Query sent={}", filter);

  filter
}

fn to_conditions(query: Query) -> Option<Vec<Document>> {
  let values = [
    ("name", query.name),
    ("city", query.city),
    ("country", query.country),
  ];
  let conditions: Vec<Document> = values
    .into_iter()
    .filter(|x| x.1.is_some())
    .map(|v| to_regex(String::from(v.0), v.1.unwrap()))
    .collect();

  if conditions.is_empty() {
    return None;
  }

  Some(conditions)
}

fn to_regex(field: String, pattern: String) -> Document {
  doc! {field: {"$regex": pattern, "$options": "i"}}
}

#[derive(Deserialize, Debug)]
pub struct Query {
  pub name: Option<String>,
  pub description: Option<String>,
  pub city: Option<String>,
  pub country: Option<String>,
}

#[derive(Serialize)]
pub struct ContactVm {
  pub website: String,
  pub email: Option<String>,
  pub phone: Option<String>,
}

impl ContactVm {
  fn from(contact: &Contact) -> Self {
    ContactVm {
      email: contact.email.clone(),
      phone: contact.phone.clone(),
      website: contact.website.clone(),
    }
  }
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
  depart_at: LocationVm,
}

impl RouteVm {
  fn from(route: &Route) -> Self {
    RouteVm {
      distance: route.distance,
      elevation: route.elevation,
      profile: route.profile.to_string(),
      description: route.description.clone(),
      depart_at: LocationVm::from(&route.depart_at),
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RideVm {
  id: String,
  name: String,
  description: String,
  route: RouteVm,
  discipline: String,
  format: String,
  contact: ContactVm,
}

impl RideVm {
  fn from(ride: Ride) -> Self {
    RideVm {
      id: ride.id.unwrap().to_string(),
      name: ride.name,
      description: ride.description,
      route: RouteVm::from(&ride.route),
      discipline: ride.discipline.to_string(),
      format: ride.format.to_string(),
      contact: ContactVm::from(&ride.contact),
    }
  }
  fn from_many(rides: Vec<Ride>) -> Vec<Self> {
    rides.into_iter().map(Self::from).collect()
  }
}
