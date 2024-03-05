use async_trait::async_trait;

use super::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    // TODO: move User to domain
    async fn save(&mut self, user: User);
}
