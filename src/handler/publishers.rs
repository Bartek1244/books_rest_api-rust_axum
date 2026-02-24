use axum::{
    Json, Router, extract::{State}, http::{StatusCode, header}, response::{IntoResponse}, routing::{get}
};

use crate::{AppState, model::publisher_model::Publisher, error::AppError, ValidatedJson};

pub fn publisher_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_publishers))
}

async fn get_all_publishers(
    State(state): State<AppState>
) -> Result<Json<Vec<Publisher>>, AppError> {
    let publishers = state
        .publisher_service
        .get_all_publishers()
        .await?;

    Ok(Json(publishers))
}