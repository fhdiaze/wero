use mongodb::{self, Database};

use crate::{
    domain::race::Race,
    infra::{config::Db, core::result::Result},
};

use super::traits::DbClient;

pub struct Client {
    database: Database,
    races: mongodb::Collection<Race>,
}

impl Client {
    pub async fn new(config: &Db) -> Result<Self> {
        let options = mongodb::options::ClientOptions::parse(&config.connection_string).await?;
        let client = mongodb::Client::with_options(options)?;
        let db = client.database(&config.db_name);
        let races = db.collection::<Race>(&config.races_collection);

        Ok(Self { races, database: db })
    }
}

impl DbClient for Client {
    fn races(&self) -> &mongodb::Collection<Race> {
        &self.races
    }

    fn database(&self) -> &Database {
        &self.database
    }
}
