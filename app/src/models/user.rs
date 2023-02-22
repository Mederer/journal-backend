use serde::{Deserialize, Serialize};

pub use crate::entities::user::Model as User;

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

impl From<User> for UserProfile {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
        }
    }
}
