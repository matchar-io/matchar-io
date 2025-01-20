use matchar_app_service::auth::google_authorize::{outbound, Error, OauthPort};
use oauth2::GoogleOauth2;

pub struct OauthRepository;

impl OauthPort for OauthRepository {
    fn new_pkce(&self) -> Result<outbound::Pkce, Error> {
        let pkce = GoogleOauth2::new(
            crate::GOOGLE_CLIENT_ID,
            crate::GOOGLE_CLIENT_SECRET,
            crate::GOOGLE_REDIRECT_URL,
        )
        .map_err(|error| Error::NewPkce(error.into()))?
        .start();

        Ok(outbound::Pkce::new(
            pkce.authorize_url,
            pkce.csrf_token,
            pkce.code_verifier,
        ))
    }
}
