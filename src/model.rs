use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String
}

#[derive(Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "cannot be empty"))]
    pub author: String
}