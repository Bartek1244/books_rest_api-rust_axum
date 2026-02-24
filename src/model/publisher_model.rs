use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Publisher {
    pub id: i32,
    pub name: String
}
