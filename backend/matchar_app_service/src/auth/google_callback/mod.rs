pub mod inbound;
pub mod outbound;
pub mod repository;

pub use repository::*;

pub struct Service<R> {
    repository: R,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("No matched")]
    NoMatched,
    #[error("Expired")]
    Expired,
    #[error("Verify error: {0}")]
    Verify(anyhow::Error),

    #[error("User info error: {0}")]
    Google(anyhow::Error),
    #[error("Oauth2 error: {0}")]
    Oauth2(anyhow::Error),
    #[error("Database error: {0}")]
    Database(anyhow::Error),
}

impl<R> Service<R>
where
    R: Repository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        inbound::Data { code, csrf_token }: inbound::Data,
    ) -> Result<outbound::Data, Error> {
        let outbound::PkceEntity {
            pkce_id,
            code_verifier,
            from_url,
            expired_at,
        } = match self
            .repository
            .pkce()
            .find_by_csrf_token(&csrf_token)
            .await?
        {
            Some(pkce) => pkce,
            None => return Err(Error::NoMatched),
        };
        if expired_at < time::OffsetDateTime::now_utc() {
            return Err(Error::Expired);
        }

        let access_token = self
            .repository
            .pkce()
            .verify_code(pkce_id, &code, &code_verifier)
            .await?;
        let user_info = self
            .repository
            .pkce()
            .user_info_in_google(&access_token)
            .await?;
        let user = match self
            .repository
            .user()
            .find_by_oauth_sub(&user_info.sub)
            .await?
        {
            Some(user) => user,
            None => {
                self.repository
                    .user()
                    .create(
                        &user_info.sub,
                        &user_info.email_address,
                        &user_info.name,
                        &user_info.image_url,
                    )
                    .await?
            }
        };

        let session_id = self.repository.session().create(user.user_id).await?;

        self.repository
            .event()
            .login_completed(user.user_id)
            .await?;

        Ok(outbound::Data {
            session_id,
            name: user.name,
            image_url: user.image_url,
            from_url,
        })
    }
}
