use mongodb::{self, options::ClientOptions, Database};

use crate::{
  domain::ride::Ride,
  infra::{config::Db, core::result::AppResult},
};

use super::traits::DbClient;

pub struct Client {
  database: Database,
  rides: mongodb::Collection<Ride>,
}

impl Client {
  pub async fn new(config: &Db) -> AppResult<Self> {
    let options = ClientOptions::parse(&config.connection_string).await?;
    let client = mongodb::Client::with_options(options)?;
    let db = client.database(&config.db_name);
    let rides = db.collection::<Ride>(&config.rides_collection);

    Ok(Self {
      rides,
      database: db,
    })
  }
}

impl DbClient for Client {
  fn rides(&self) -> &mongodb::Collection<Ride> {
    &self.rides
  }

  fn database(&self) -> &Database {
    &self.database
  }
}
