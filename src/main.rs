mod shared;
mod user;

use axum::{
    extract::Path,
    http::StatusCode,
    response::Html,
    routing::{delete, get, post},
    Json, Router,
};
use user::{CreateUserRequest, User, UserResponse};
use uuid::Uuid;

async fn handler_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<UserResponse>) {
    let user_response = User::create_user(payload).unwrap();
    (StatusCode::CREATED, Json(user_response))
}

async fn handler_delete_user(Path(id): Path<Uuid>) -> (StatusCode, ()) {
    match User::delete_user(id) {
        Ok(true) => (StatusCode::NO_CONTENT, ()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, ()),
    }
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/users", post(handler_user))
        .route("/users/:id", delete(handler_delete_user));

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
