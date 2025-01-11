use database::{ConnectionPool, DatabaseDriver};
use matchar_app_service::auth::google_callback::{
    AccessToken, Code, CodeVerifier, CsrfToken, Error, PkceEntity, Repository, UserEntity, UserInfo,
};
use refinement::{EmailAddress, SessionId, UserId};

pub struct Adapter {
    pool: ConnectionPool,
}

impl Adapter {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl Repository for Adapter {
    async fn find_pkce_by_csrf_token(
        &self,
        csrf_token: &CsrfToken,
    ) -> Result<Option<PkceEntity>, Error> {
        std::todo!();
    }

    async fn verify_code(
        &self,
        code: &Code,
        code_verifier: &CodeVerifier,
    ) -> Result<AccessToken, Error> {
        std::todo!();
    }

    async fn user_info_in_google(&self, access_token: &AccessToken) -> Result<UserInfo, Error> {
        std::todo!();
    }

    async fn find_user_by_oauth_sub(&self, sub: &str) -> Result<Option<UserEntity>, Error> {
        std::todo!();
    }

    async fn new_user(
        &self,
        email_address: EmailAddress,
        name: Option<String>,
        image_url: Option<String>,
    ) -> Result<UserEntity, Error> {
        std::todo!();
    }

    async fn new_session(
        &self,
        user_id: UserId,
        name: String,
        image_url: String,
    ) -> Result<SessionId, Error> {
        std::todo!();
    }

    async fn logged_in_event(&self, user_id: UserId) -> Result<(), Error> {
        std::todo!();
    }
}
