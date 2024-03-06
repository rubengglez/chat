#![deny(missing_debug_implementations)]

mod shared;
mod user;

use std::{sync::Arc, time::Duration};
use log::{info, debug};
use dotenv::dotenv;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post},
    Json, Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};
use user::{domain::UserRepository, CreateUserRequest, User, UserResponse};
use uuid::Uuid;

use crate::user::infrastructure::PostgresqlUserRepository;

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

async fn handler_user(
    State(dependencies): State<Dependencies>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    let user_response = User::create_user(payload).unwrap();
    (StatusCode::CREATED, Json(user_response))
}

async fn handler_delete_user(Path(id): Path<Uuid>) -> (StatusCode, ()) {
    match User::delete_user(id) {
        Ok(true) => (StatusCode::NO_CONTENT, ()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, ()),
    }
}

#[derive(Clone)]
struct Dependencies {
    pub user_repository: Arc<dyn UserRepository>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    info!("Starting chat server");
    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    debug!("DB connection: {}", db_connection_str);

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("can't migrate");

    let dependencies = Dependencies {
        user_repository: Arc::new(PostgresqlUserRepository::new(pool.clone())),
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/users", post(handler_user))
        .route("/users/:id", delete(handler_delete_user))
        .with_state(dependencies)
        .with_state(pool);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(
        r#"
    <!doctype html> <html>     <head>         <title>My Chat</title>     </head>     <body>         <h3>Rust Microservice</h3> <p>Used to learn more about Rust microservices and Rust in general.</p>    </body> </html>
        "#,
    )
}
