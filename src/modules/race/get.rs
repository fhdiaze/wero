use std::str::FromStr;

use crate::{
    domain::race::Race,
    infra::{core::result::Result, db::traits::DynDbClient, error::AppError},
};

use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
    /// The id of the race
    id: String,
}

impl Query {
    pub fn new(id: String) -> Self {
        Query { id }
    }
}

pub async fn handle(db: DynDbClient, query: Query) -> Result<RaceVm> {
    let id = ObjectId::from_str(&query.id).unwrap();
    let filter = doc! {"_id": id };
    let opt_race = db.races().find_one(Some(filter), None).await?;

    match opt_race {
        Some(race) => Ok(RaceVm::from(&race)),
        None => Err(AppError::NotFound(format!(
            "No race was found with id={}",
            query.id
        ))),
    }
}

#[derive(Serialize)]
pub struct RaceVm {
    id: String,
    name: String,
}

impl RaceVm {
    fn from(race: &Race) -> Self {
        RaceVm {
            id: race.id.clone().unwrap(),
            name: race.name.clone(),
        }
    }
}
