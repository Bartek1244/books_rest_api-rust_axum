use sqlx::{postgres::PgPoolOptions};
use axum::{
    Router
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::{db_repository::DbRepository, json_validation::ValidatedJson, handlers::app_router, model::book_model::Book, model::book_model::CreateBook};

mod handlers;
mod model;
mod db_repository;
mod error;
mod json_validation;

#[derive(Clone)]
pub struct AppState {
    pub db_repo: DbRepository, 
}

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
    
    let db_repo = DbRepository::new(pool);
    let state = AppState { db_repo };

    let app = Router::new()
        .merge(app_router())
        .with_state(state)
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