use crate::domain::ride::Ride;
use mongodb::{Collection, Database};
use std::sync::Arc;

pub type DynDbClient = Arc<dyn DbClient + Send + Sync>;

pub trait DbClient {
  fn rides(&self) -> &Collection<Ride>;
  fn database(&self) -> &Database;
}
