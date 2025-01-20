mod oauth;
mod session;

use database::ConnectionPool;
use matchar_app_service::auth::google_authorize::Port;

pub struct Repository {
    oauth: oauth::OauthRepository,
    session: session::SessionRepository,
}

impl Repository {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self {
            oauth: oauth::OauthRepository,
            session: session::SessionRepository::new(pool),
        }
    }
}

impl Port for Repository {
    type Oauth = oauth::OauthRepository;
    type Session = session::SessionRepository;

    #[inline]
    fn oauth(&self) -> &Self::Oauth {
        &self.oauth
    }

    #[inline]
    fn session(&self) -> &Self::Session {
        &self.session
    }
}
