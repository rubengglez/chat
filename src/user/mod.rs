pub mod domain;
pub mod infrastructure;

use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub id: String,
}

// TODO: we can implement the fmt::Display trait to
// to convert to string (or serialize) this struct
#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub username: String,
    pub id: usize,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
}

impl User {
    pub fn create_user(data: CreateUserRequest) -> Result<UserResponse, Infallible> {
        debug!("creating a user: {}", data.username);

        let user = UserResponse {
            id: 1337,
            username: data.username,
        };

        Ok(user)
    }

    pub fn delete_user(id: Uuid) -> Result<bool, Infallible> {
        Ok(true)
    }
}
