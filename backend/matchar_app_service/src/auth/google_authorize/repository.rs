use super::{inbound, outbound, Error};

pub trait Repository: Sync + Send + 'static {
    type Oauth: OauthRepository;

    type Session: SessionRepository;

    fn oauth(&self) -> &Self::Oauth;

    fn session(&self) -> &Self::Session;
}

pub trait OauthRepository {
    fn new_pkce(&self) -> Result<outbound::Pkce, Error>;
}

pub trait SessionRepository {
    async fn store_pkce(
        &self,
        csrf_token: &outbound::CsrfToken,
        code_verifier: &outbound::CodeVerifier,
        from_url: inbound::FromUrl,
    ) -> Result<(), Error>;
}
