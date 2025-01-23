use super::{inbound, outbound, OauthPort, Port, SessionPort};

pub trait UseCase {
    async fn google_authorize(&self, data: inbound::Data) -> Result<outbound::Data, Error>;
}

pub struct Service<P> {
    port: P,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create PKCE: {0}")]
    NewPkce(anyhow::Error),
    #[error("Failed to store PKCE: {0}")]
    StorePkce(#[from] anyhow::Error),
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
    async fn google_authorize(
        &self,
        inbound::Data { from_url }: inbound::Data,
    ) -> Result<outbound::Data, Error> {
        let outbound::Pkce {
            redirect_url,
            csrf_token,
            code_verifier,
        } = self.port.oauth().new_pkce()?;
        self.port
            .session()
            .store_pkce(&csrf_token, &code_verifier, from_url)
            .await?;

        Ok(outbound::Data { redirect_url })
    }
}
