use super::User;

pub trait UserRepository {
    // TODO: move User to domain
    fn save(&mut self, user: User) {}
}
