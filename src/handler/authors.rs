use axum::{
    Json, Router, extract::{Path, State}, http::{StatusCode, header}, response::{IntoResponse}, routing::{get}
};

use crate::{AppState, model::author_model::Author, error::AppError, ValidatedJson};

pub fn author_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_authors))
}

async fn get_all_authors(
    State(state): State<AppState>
) -> Result<Json<Vec<Author>>, AppError> {
    let authors = state
        .author_service
        .get_all_authors()
        .await?;

    Ok(Json(authors))
}