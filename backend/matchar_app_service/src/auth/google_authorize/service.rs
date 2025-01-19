use super::{inbound, outbound, OauthRepository, Repository, SessionRepository};

pub trait UseCase {
    async fn google_authorize(&self, data: inbound::Data) -> Result<outbound::Data, Error>;
}

pub struct Service<R> {
    repository: R,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create PKCE: {0}")]
    Pkce(anyhow::Error),
}

impl<R> Service<R> {
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> UseCase for Service<R>
where
    R: Repository,
{
    async fn google_authorize(
        &self,
        inbound::Data { from_url }: inbound::Data,
    ) -> Result<outbound::Data, Error> {
        let outbound::Pkce {
            redirect_url,
            csrf_token,
            code_verifier,
        } = self.repository.oauth().new_pkce()?;
        self.repository
            .session()
            .with_pkce(&csrf_token, &code_verifier, from_url)
            .await
            .map_err(|error| Error::Pkce(error.into()))?;

        Ok(outbound::Data { redirect_url })
    }
}
