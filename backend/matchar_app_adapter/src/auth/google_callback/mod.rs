mod event;
mod pkce;
mod session;
mod user;

use database::ConnectionPool;
use matchar_app_service::auth::google_callback::{Error, Repository};

pub struct Adapter {
    pkce: pkce::PkceAdapter,
    user: user::UserAdapter,
    session: session::SessionAdapter,
    event: event::EventAdapter,
}

impl Adapter {
    pub fn new(pool: ConnectionPool) -> Result<Self, Error> {
        Ok(Self {
            pkce: pkce::PkceAdapter::new(pool.clone())?,
            user: user::UserAdapter::new(pool.clone()),
            session: session::SessionAdapter::new(pool),
            event: event::EventAdapter,
        })
    }
}

impl Repository for Adapter {
    type Pkce = pkce::PkceAdapter;

    type User = user::UserAdapter;

    type Session = session::SessionAdapter;

    type Event = event::EventAdapter;

    #[inline]
    fn pkce(&self) -> &Self::Pkce {
        &self.pkce
    }

    #[inline]
    fn user(&self) -> &Self::User {
        &self.user
    }

    #[inline]
    fn session(&self) -> &Self::Session {
        &self.session
    }

    #[inline]
    fn event(&self) -> &Self::Event {
        &self.event
    }
}
