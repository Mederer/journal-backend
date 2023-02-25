use serde::{Deserialize, Serialize};

use crate::entities::user;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl From<user::Model> for UserProfile {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
        }
    }
}
