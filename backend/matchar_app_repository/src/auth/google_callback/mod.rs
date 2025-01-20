mod event;
mod pkce;
mod session;
mod user;

use database::ConnectionPool;
use matchar_app_service::auth::google_callback::{Error, Port};

pub struct Repository {
    pkce: pkce::PkceRepository,
    user: user::UserRepository,
    session: session::SessionRepository,
    event: event::EventRepository,
}

impl Repository {
    pub fn new(pool: ConnectionPool) -> Result<Self, Error> {
        Ok(Self {
            pkce: pkce::PkceRepository::new(pool.clone())?,
            user: user::UserRepository::new(pool.clone()),
            session: session::SessionRepository::new(pool),
            event: event::EventRepository,
        })
    }
}

impl Port for Repository {
    type Pkce = pkce::PkceRepository;

    type User = user::UserRepository;

    type Session = session::SessionRepository;

    type Event = event::EventRepository;

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
