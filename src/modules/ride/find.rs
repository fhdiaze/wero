use crate::domain::contact::Contact;
use crate::domain::details::Details;
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
  let page = cursor.page;
  let rides = find_rides(db, cursor).await?;
  let rides_vm: Vec<RideVm> =
    rides.into_iter().map(|r| RideVm::from(&r)).collect();
  let size = rides_vm.len();

  Ok(Page::new(rides_vm, page, size))
}

async fn find_rides(
  db: DynDbClient,
  cursor: Cursor<Query>,
) -> Result<Vec<Ride>> {
  let offset = cursor.page * cursor.size;
  let options = FindOptions::builder()
    .sort(doc! { "start_at": 1, "_id": 1 })
    .skip(offset as u64)
    .limit(cursor.size as i64)
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

  if let Some(query) = cursor.query {
    let mut conditions: Vec<Document> = vec![];

    if let Some(name) = query.name.filter(|x| !x.is_empty()) {
      conditions.push(doc! {"name": {"$regex": name, "$options": "i"}});
    }

    if let Some(city) = query.city.filter(|x| !x.is_empty()) {
      conditions
        .push(doc! { "location.city": {"$regex": city, "$options": "i"} });
    }

    if let Some(country) = query.country.filter(|x| !x.is_empty()) {
      conditions.push(
        doc! { "location.country": {"$regex": country, "$options": "i"} },
      );
    }

    if !conditions.is_empty() {
      filter.insert("$or", conditions);
    }
  }

  tracing::info!("Query sent={}", filter);

  filter
}

#[derive(Serialize)]
pub struct ContactVm {
  pub website: String,
  pub email: String,
  pub phone: String,
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
  start: LocationVm,
}

impl RouteVm {
  fn from(route: &Route) -> Self {
    RouteVm {
      distance: route.distance,
      elevation: route.elevation,
      profile: route.profile.to_string(),
      description: route.description.clone(),
      start: LocationVm::from(&route.start),
    }
  }
}

#[derive(Serialize)]
pub struct DetailsVm {
  route: RouteVm,
  discipline: String,
  format: String,
}

impl DetailsVm {
  fn from(details: &Details) -> Self {
    DetailsVm {
      route: RouteVm::from(&details.route),
      discipline: details.discipline.to_string(),
      format: details.format.to_string(),
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
  details: DetailsVm,
  contact: ContactVm,
}

impl RideVm {
  fn from(ride: &Ride) -> Self {
    RideVm {
      id: ride.id.unwrap().to_string(),
      name: ride.name.clone(),
      description: ride.description.clone(),
      start_at: ride.start_at,
      details: DetailsVm::from(&ride.details),
      contact: ContactVm::from(&ride.contact),
    }
  }
}
