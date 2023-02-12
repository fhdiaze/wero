use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
  pub distance: f64,
  pub elevation: i32,
  pub profile: String,
  pub description: String,
}

impl Route {
  fn new(
    distance: f64,
    elevation: i32,
    profile: String,
    description: String,
  ) -> Self {
    Route {
      distance,
      elevation,
      profile,
      description,
    }
  }
}
