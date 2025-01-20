use super::{inbound, outbound, Error};

pub trait Port: Sync + Send + 'static {
    type Oauth: OauthPort;

    type Session: SessionPort;

    fn oauth(&self) -> &Self::Oauth;

    fn session(&self) -> &Self::Session;
}

pub trait OauthPort {
    fn new_pkce(&self) -> Result<outbound::Pkce, Error>;
}

pub trait SessionPort {
    async fn store_pkce(
        &self,
        csrf_token: &outbound::CsrfToken,
        code_verifier: &outbound::CodeVerifier,
        from_url: inbound::FromUrl,
    ) -> Result<(), Error>;
}
