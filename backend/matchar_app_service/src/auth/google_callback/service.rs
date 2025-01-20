use super::{inbound, outbound, EventPort, PkcePort, Port, SessionPort, UserPort};

pub trait UseCase {
    async fn google_callback(&self, data: inbound::Data) -> Result<outbound::Data, Error>;
}

pub struct Service<P> {
    port: P,
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

impl<P> Service<P> {
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P> UseCase for Service<P>
where
    P: Port,
{
    async fn google_callback(
        &self,
        inbound::Data { code, csrf_token }: inbound::Data,
    ) -> Result<outbound::Data, Error> {
        let outbound::PkceEntity {
            pkce_id,
            code_verifier,
            from_url,
            expired_at,
        } = match self.port.pkce().find_by_csrf_token(&csrf_token).await? {
            Some(pkce) => pkce,
            None => return Err(Error::NoMatched),
        };
        if expired_at < time::OffsetDateTime::now_utc() {
            return Err(Error::Expired);
        }

        let access_token = self
            .port
            .pkce()
            .verify_code(pkce_id, &code, &code_verifier)
            .await?;
        let user_info = self.port.pkce().user_info_in_google(&access_token).await?;
        let user = match self.port.user().find_by_oauth_sub(&user_info.sub).await? {
            Some(user) => user,
            None => {
                self.port
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

        let session_id = self.port.session().create(user.user_id).await?;

        self.port.event().login_completed(user.user_id).await?;

        Ok(outbound::Data {
            session_id,
            name: user.name,
            image_url: user.image_url,
            from_url,
        })
    }
}
