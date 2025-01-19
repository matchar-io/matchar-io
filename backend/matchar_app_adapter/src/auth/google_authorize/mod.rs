mod oauth;
mod session;

use database::ConnectionPool;
use matchar_app_service::auth::google_authorize::Repository;

pub struct Adapter {
    oauth: oauth::OauthAdapter,
    session: session::SessionAdapter,
}

impl Adapter {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self {
            oauth: oauth::OauthAdapter,
            session: session::SessionAdapter::new(pool),
        }
    }
}

impl Repository for Adapter {
    type Oauth = oauth::OauthAdapter;
    type Session = session::SessionAdapter;

    #[inline]
    fn oauth(&self) -> &Self::Oauth {
        &self.oauth
    }

    #[inline]
    fn session(&self) -> &Self::Session {
        &self.session
    }
}
