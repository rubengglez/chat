use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::domain::UserRepository;

pub struct PostgresqlUserRepository {
    db: Pool<Postgres>,
}

impl PostgresqlUserRepository {
    pub fn new(db: Pool<Postgres>) -> PostgresqlUserRepository {
        PostgresqlUserRepository { db }
    }
}

#[async_trait]
impl UserRepository for PostgresqlUserRepository {
    async fn save(&mut self, user: super::User) {
        sqlx::query("INSERT INTO users VALUES (?, ?)")
            .bind(user.username)
            .bind(user.password_hash)
            .execute(&self.db)
            .await
            .unwrap();
    }
}
