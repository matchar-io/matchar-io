use database::DatabaseDriver;
use matchar_app_service::auth::google_callback::{
    AccessToken, Error, Repository, SessionToken, UserEntity, UserInfo, UserToken,
};
use refinement::{EmailAddress, SessionId, UserId};

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
    async fn find_pkce_by_code(
        &self,
        code: &str,
    ) -> Result<matchar_app_service::auth::google_callback::PkceEntity, Error> {
        std::todo!();
    }

    async fn verify_code(&self, code: &str, csrf_token: &str) -> Result<AccessToken, Error> {
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
