use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{location::Location, profile::Profile};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
  pub start_at: DateTime<Utc>,
  pub distance: f64,
  pub elevation: i32,
  pub profile: Profile,
  pub description: String,
  pub depart_at: Location,
}
