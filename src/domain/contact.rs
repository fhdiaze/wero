use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
  pub website: String,
  pub email: Option<String>,
  pub phone: Option<String>,
}
