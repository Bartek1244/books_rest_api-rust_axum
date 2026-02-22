use sqlx::{postgres::PgPoolOptions, PgPool};
use axum::{
    Json, Router, extract::{Path, State}, http::{StatusCode, header}, response::{IntoResponse}, routing::{get}
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use error::AppError;
use model::{Book, CreateBook};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::json_validation::ValidatedJson;

mod model;
mod error;
mod json_validation;


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,tower_http=debug,sqlx=debug".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL")
        .expect("couldn't resolve DATABSE_URL from .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("couldn't establish database pool connection");

    let app = Router::new()
        .route("/", get(root))
        .route("/books", get(get_books).post(create_book))
        .route("/books/{id}", get(get_book_by_id))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("couldn't bind port to listener");

    println!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("server down");
}

async fn root() -> &'static str {
    "Hello rust api!"
}

async fn get_books(
    State(pool): State<PgPool>
) -> Result<Json<Vec<Book>>, AppError> {
    let books = sqlx::query_as!(
        Book,
        "SELECT id, title, author FROM test.book"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(books))
}

async fn get_book_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<Json<Book>, AppError> {
    let book = sqlx::query_as!(
        Book,
        "SELECT id, title, author FROM test.book WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(book))
}

async fn create_book(
    State(pool): State<PgPool>,
    ValidatedJson(payload): ValidatedJson<CreateBook>
) -> Result<impl IntoResponse, AppError> {
    let created_book = sqlx::query_as!(
        Book,
        "INSERT INTO test.book (title, author) VALUES ($1, $2) RETURNING id, title, author",
        payload.title,
        payload.author
    )
    .fetch_one(&pool)
    .await?;

    Ok((
        StatusCode::CREATED, 
        [((header::LOCATION), format!("/books/{}", created_book.id))],
        Json(created_book)
    ))
}