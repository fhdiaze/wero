use serde::{Deserialize, Serialize};

use super::{discipline::Discipline, format::Format, route::Route};

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
  pub route: Route,
  pub discipline: Discipline,
  pub format: Format,
}
