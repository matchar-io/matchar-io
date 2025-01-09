use matchar_app_service::auth::google_authorize::{
    CodeVerifier, CsrfToken, Error, Pkce, RedirectUrl, Repository,
};
use oauth2::GoogleOauth2;

pub struct Adapter(());

impl Adapter {
    pub const fn new() -> Self {
        Self(())
    }
}

impl Repository for Adapter {
    fn new_pkce(&self) -> Result<Pkce, Error> {
        let pkce = GoogleOauth2::new(
            crate::GOOGLE_CLIENT_ID,
            crate::GOOGLE_CLIENT_SECRET,
            crate::GOOGLE_REDIRECT_URL,
        )
        .map_err(|error| Error::Pkce(error.into()))?
        .start();

        Ok(Pkce {
            redirect_url: RedirectUrl(pkce.authorize_url),
            csrf_token: CsrfToken(pkce.csrf_token),
            code_verifier: CodeVerifier(pkce.code_verifier),
        })
    }

    async fn pkce_session(
        &self,
        csrf_token: CsrfToken,
        code_verifier: CodeVerifier,
    ) -> Result<(), Error> {
        std::todo!();
    }
}
