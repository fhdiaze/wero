use crate::domain::race::Race;
use mongodb::{Collection, Database};
use std::sync::Arc;

pub type DynDbClient = Arc<dyn DbClient + Send + Sync>;

pub trait DbClient {
    fn races(&self) -> &Collection<Race>;
    fn database(&self) -> &Database;
}
