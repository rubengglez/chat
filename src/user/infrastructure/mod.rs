use sqlx::{Pool, Postgres};

use super::domain::UserRepository;

pub struct PostgresqlUserRepository<'a> {
    db: &'a Pool<Postgres>,
}

impl<'a> PostgresqlUserRepository<'a> {
    fn new(db: &Pool<Postgres>) -> PostgresqlUserRepository {
        PostgresqlUserRepository { db }
    }
}

impl UserRepository for PostgresqlUserRepository {
	fn save(&mut self, user: super::User) {
	}
}