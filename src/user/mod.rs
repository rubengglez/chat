use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub id: usize,
}

pub struct User {}

impl User {
    pub fn create_user(data: CreateUserRequest) -> Result<UserResponse, Infallible> {
        let user = UserResponse {
            id: 1337,
            username: data.username,
        };

        Ok(user)
    }
}
