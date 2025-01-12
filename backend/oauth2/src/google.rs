pub struct GoogleOauth2 {
    client: oauth2::basic::BasicClient,
}

pub struct Pkce {
    pub authorize_url: String,
    pub csrf_token: String,
    pub code_verifier: String,
}

pub struct AccessToken(String);

pub struct UserInfo {
    pub sub: String,
    pub email_address: String,
    pub name: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid authorization URL")]
    InvalidAuthUrl,
    #[error("Invalid token URL")]
    InvalidTokenUrl,
    #[error("Invalid redirect URL")]
    InvalidRedirectUrl,

    #[error("Failed to verify")]
    VerificationFailed,

    #[error("Failed to get user info from response")]
    UserInfoResponse,
    #[error("Failed to get user info from body")]
    UserInfoBody,
    #[error("Failed to get user info from JSON")]
    UserInfoJson,
}

impl GoogleOauth2 {
    pub fn new(
        client_id: &'static str,
        client_secret: &'static str,
        redirect_url: &'static str,
    ) -> Result<Self, Error> {
        const GOOGLE_AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/auth";
        const GOOGLE_TOKEN_URL: &'static str = "https://oauth2.googleapis.com/token";

        let client_id = oauth2::ClientId::new(client_id.to_owned());
        let client_secret = oauth2::ClientSecret::new(client_secret.to_owned());
        let auth_url =
            oauth2::AuthUrl::new(GOOGLE_AUTH_URL.to_owned()).map_err(|_| Error::InvalidAuthUrl)?;
        let token_url = oauth2::TokenUrl::new(GOOGLE_TOKEN_URL.to_owned())
            .map_err(|_| Error::InvalidTokenUrl)?;
        let redirect_url = oauth2::RedirectUrl::new(redirect_url.to_owned())
            .map_err(|_| Error::InvalidRedirectUrl)?;

        let client = oauth2::basic::BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self { client })
    }

    pub fn start(&self) -> Pkce {
        let (challenge, code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let (authorize_url, csrf_token) = self
            .client
            .authorize_url(oauth2::CsrfToken::new_random)
            .set_pkce_challenge(challenge)
            .url();

        Pkce {
            authorize_url: authorize_url.to_string(),
            csrf_token: csrf_token.secret().to_owned(),
            code_verifier: code_verifier.secret().to_owned(),
        }
    }

    pub async fn verify(&self, code: &str, code_verifier: &str) -> Result<AccessToken, Error> {
        use oauth2::TokenResponse;

        let code = oauth2::AuthorizationCode::new(code.to_owned());
        let code_verifier = oauth2::PkceCodeVerifier::new(code_verifier.to_owned());

        let response = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(code_verifier)
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|_| Error::VerificationFailed)?;
        let access_token = AccessToken(response.access_token().secret().to_owned());

        Ok(access_token)
    }

    pub async fn user_info(access_token: AccessToken) -> Result<UserInfo, Error> {
        use reqwest::header::AUTHORIZATION;

        const GOOGLE_USER_INFO_URL: &'static str = "https://www.googleapis.com/oauth2/v3/userinfo";

        #[derive(Deserialize)]
        struct Data {
            sub: String,
            email: String,
            name: Option<String>,
            picture: Option<String>,
        }

        let response: Data = reqwest::Client::new()
            .get(GOOGLE_USER_INFO_URL)
            .header(AUTHORIZATION, format!("Bearer {}", access_token.0))
            .send()
            .await
            .map_err(|_| Error::UserInfoResponse)?
            .json()
            .await
            .map_err(|_| Error::UserInfoBody)?;

        Ok(UserInfo {
            sub: response.sub,
            email_address: response.email,
            name: response.name,
            image_url: response.picture,
        })
    }
}

impl AccessToken {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[test]
fn google_pkce_len() -> anyhow::Result<()> {
    let pkce = GoogleOauth2::new(
        "client_id",
        "client_secret",
        "http://localhost:8080/auth/google/callback",
    )?
    .start();

    assert_eq!(pkce.csrf_token.len(), 22);
    assert_eq!(pkce.code_verifier.len(), 43);

    Ok(())
}
