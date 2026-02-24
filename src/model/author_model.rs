use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String
}
