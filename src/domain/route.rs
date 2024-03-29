use serde::{Deserialize, Serialize};

use super::{location::Location, profile::Profile};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
  pub distance: f64,
  pub elevation: i32,
  pub profile: Profile,
  pub description: String,
  pub depart_at: Location,
}
