use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Discipline {
    Road,
    Mountain,
    Track,
    Gravel,
}
