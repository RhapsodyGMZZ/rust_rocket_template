use rocket::serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WSMessage {
    pub r#type: String,
    pub from: String,
    pub to: String,
    pub data: String,
}