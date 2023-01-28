use crate::{
    domain::{discipline::Discipline, location::Location, race::Race},
    infra::{core::result::Result, db::traits::DynDbClient},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Command {
    pub name: String,
    pub distance: f64,
    pub date: DateTime<Utc>,
    pub discipline: Discipline,
    pub location: Location,
    pub image: String,
}

pub async fn handle(db: DynDbClient, cmd: Command) -> Result<RaceVm> {
    let race = Race::new(
        None,
        cmd.name,
        cmd.distance,
        cmd.discipline,
        cmd.location,
        cmd.image,
    );
    let result = db.races().insert_one(race, None).await?;
    let race_id = result.inserted_id.to_string();

    Ok(RaceVm::new(race_id))
}

#[derive(Serialize)]
pub struct RaceVm {
    id: String,
}

impl RaceVm {
    fn new(id: String) -> Self {
        RaceVm { id }
    }
}
