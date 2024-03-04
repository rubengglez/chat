mod shared;
mod user;

use std::time::Duration;

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

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

async fn handler_user(
    State(pool): State<PgPool>,
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

struct DependenciesBuilder<'a> {
    pub user_repository: &'a dyn UserRepository,
}


#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:20000/chat".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!("db/migrations").run(&pool).await.expect("can't migrate");

    let dependencies = Dependencies{
        user_repository: PostgresqlUserRepository::new(&pool),
    }

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/users", post(handler_user))
        .route("/users/:id", delete(handler_delete_user))
        .with_state(pool)
        .with_state(dependencies);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
