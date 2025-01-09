use database::DatabaseDriver;
use matchar_app_service::auth::google_callback::{
    AccessToken, Error, Repository, SessionToken, UserEntity, UserInfo, UserToken,
};
use refinement::EmailAddress;

pub struct Adapter<D: DatabaseDriver> {
    driver: D,
}

impl<D> Adapter<D>
where
    D: DatabaseDriver,
{
    pub const fn new(driver: D) -> Self {
        Self { driver }
    }
}

impl<D> Repository for Adapter<D>
where
    D: DatabaseDriver,
{
    async fn verify_code(
        &self,
        code: String,
        code_verifier: String,
        csrf_token: String,
    ) -> Result<AccessToken, Error> {
        std::todo!();
    }

    async fn user_info_in_google(&self, access_token: AccessToken) -> Result<UserInfo, Error> {
        std::todo!();
    }

    async fn find_user(&self, sub: String) -> Result<Option<UserEntity>, Error> {
        std::todo!();
    }

    async fn new_user(
        &self,
        email_address: EmailAddress,
        name: String,
        image_url: Option<String>,
    ) -> Result<UserEntity, Error> {
        std::todo!();
    }

    async fn new_session(
        &self,
        user_id: String,
        name: String,
        image_url: String,
    ) -> Result<SessionId, Error> {
        std::todo!();
    }

    fn session_token(&self, session_id: SessionId) -> SessionToken {
        std::todo!();
    }

    async fn logged_in_event(&self, user_id: UserId) -> Result<(), Error> {
        std::todo!();
    }
}
