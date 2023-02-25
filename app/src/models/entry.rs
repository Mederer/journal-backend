use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewEntry {
    pub title: String,
    pub body: String,
}
