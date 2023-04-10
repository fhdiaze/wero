use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
  pub address: String,
  pub city: String,
  pub region: String,
  pub country: String,
}
