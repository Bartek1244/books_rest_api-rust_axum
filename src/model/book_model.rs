use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub isbn: String,
    pub title: String,
    pub publish_year: Option<i32>,
    pub author_id: Option<i32>,
    pub publisher_id: Option<i32>,
    pub page_count: Option<i32>
}

#[derive(Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "cannot be empty"))]
    pub author: String
}