use axum::Router;
use crate::{AppState};

pub mod books;
pub mod authors;
pub mod publishers;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("/books", books::book_router())
        .nest("/authors", authors::author_router())
        .nest("/publishers", publishers::publisher_router())
}