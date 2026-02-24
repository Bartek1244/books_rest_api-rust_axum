use axum::Router;
use crate::{AppState};

pub mod books;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .merge(books::router())
}