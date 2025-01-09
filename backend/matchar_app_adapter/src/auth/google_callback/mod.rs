use database::DatabaseDriver;
use matchar_app_service::auth::google_callback::{Error, Repository, UserToken};

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
    async fn verify_code(&self, code: String, code_verifier: String) -> Result<UserToken, Error> {
        std::todo!();
    }

    async fn logged_in_event(&self, token: UserToken) -> Result<(), Error> {
        std::todo!();
    }
}
