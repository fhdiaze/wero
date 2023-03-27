use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
  pub distance: f64,
  pub elevation: i32,
  pub profile: String,
  pub description: String,
}
