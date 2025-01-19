mod user;

use database::ConnectionPool;
use matchar_app_service::me::information::Repository;

pub struct Adapter {
    user: user::UserAdapter,
}

impl Adapter {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            user: user::UserAdapter::new(pool),
        }
    }
}

impl Repository for Adapter {
    type User = user::UserAdapter;

    #[inline]
    fn user(&self) -> &user::UserAdapter {
        &self.user
    }
}
