use axum::{
    Json, Router, extract::{Path, State}, http::{StatusCode, header}, response::{IntoResponse}, routing::{get}
};

use crate::{AppState, Book, CreateBook, error::AppError, ValidatedJson};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/books", get(get_books).post(create_book))
        .route("/books/{id}", get(get_book_by_id))
}

async fn root() -> &'static str {
    "Hello rust api!"
}

async fn get_books(
    State(state): State<AppState>
) -> Result<Json<Vec<Book>>, AppError> {
    let books = state
        .book_service
        .get_all_books()
        .await?;

    Ok(Json(books))
}

async fn get_book_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<Json<Book>, AppError> {
    let book = state
        .book_service
        .get_book_by_id(id)
        .await?;

    Ok(Json(book))
}

async fn create_book(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateBook>
) -> Result<impl IntoResponse, AppError> {
    let created_book = state
        .book_service
        .create_book(&payload)
        .await?;

    Ok((
        StatusCode::CREATED, 
        [((header::LOCATION), format!("/books/{}", created_book.id))],
        Json(created_book)
    ))
}