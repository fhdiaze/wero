use crate::{
  domain::{location::Location, ride::Ride, route::Route},
  infra::{core::result::Result, db::traits::DynDbClient, error::AppError},
};
use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
  /// The id of the ride
  id: ObjectId,
}

pub async fn handle(db: DynDbClient, query: Query) -> Result<RideVm> {
  let filter = doc! {"_id": query.id };
  let opt_ride = db.rides().find_one(Some(filter), None).await?;

  match opt_ride {
    Some(ride) => Ok(RideVm::from(&ride)),
    None => Err(AppError::NotFound(format!(
      "No ride was found with id={}",
      query.id
    ))),
  }
}

#[derive(Serialize)]
pub struct RideVm {
  id: String,
  name: String,
  route: RouteVm,
  location: LocationVm,
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
  pub distance: f64,
  pub elevation: i32,
  pub profile: String,
  pub description: String,
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

impl RideVm {
  fn from(ride: &Ride) -> Self {
    RideVm {
      id: ride.id.unwrap().to_string(),
      name: ride.name.clone(),
      route: RouteVm::from(&ride.route),
      location: LocationVm::from(&ride.location),
    }
  }
}
