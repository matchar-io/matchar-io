mod user;

use database::ConnectionPool;
use matchar_app_service::user::information::Port;

pub struct Repository {
    user: user::UserRepository,
}

impl Repository {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            user: user::UserRepository::new(pool),
        }
    }
}

impl Port for Repository {
    type User = user::UserRepository;

    #[inline]
    fn user(&self) -> &user::UserRepository {
        &self.user
    }
}
