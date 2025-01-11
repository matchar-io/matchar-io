use refinement::{EmailAddress, SessionId, UserId};

pub trait Repository {
    async fn find_pkce_by_csrf_token(
        &self,
        csrf_token: &CsrfToken,
    ) -> Result<Option<PkceEntity>, Error>;

    async fn verify_code(
        &self,
        code: &Code,
        code_verifier: &CodeVerifier,
    ) -> Result<AccessToken, Error>;

    async fn user_info_in_google(&self, access_token: &AccessToken) -> Result<UserInfo, Error>;

    async fn find_user_by_oauth_sub(&self, sub: &str) -> Result<Option<UserEntity>, Error>;

    async fn new_user(
        &self,
        email_address: EmailAddress,
        name: Option<String>,
        image_url: Option<String>,
    ) -> Result<UserEntity, Error>;

    async fn new_session(
        &self,
        user_id: UserId,
        name: String,
        image_url: String,
    ) -> Result<SessionId, Error>;

    async fn logged_in_event(&self, user_id: UserId) -> Result<(), Error>;
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Data {
    pub session_id: SessionId,
}

pub struct CsrfToken(String);

pub struct Code(String);

pub struct CodeVerifier(String);

pub struct PkceEntity {
    pub code_verifier: CodeVerifier,
    pub expired_at: time::OffsetDateTime,
}

pub struct AccessToken(pub String);

pub struct UserInfo {
    pub sub: String,
    pub email_address: EmailAddress,
    pub name: Option<String>,
    pub image_url: Option<String>,
}

pub struct UserEntity {
    pub user_id: UserId,
    pub email_address: EmailAddress,
    pub name: String,
    pub image_url: String,
}

pub struct SessionToken(pub String);

pub struct UserToken(pub String);

#[derive(Debug, Error)]
pub enum Error {
    #[error("No matched")]
    NoMatched,
    #[error("Invalid CSRF token")]
    Expired,
}

impl<R> Service<R>
where
    R: Repository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, code: String, csrf_token: String) -> Result<Data, Error> {
        let code = Code(code);
        let csrf_token = CsrfToken(csrf_token);
        let PkceEntity {
            code_verifier,
            expired_at,
        } = match self.repository.find_pkce_by_csrf_token(&csrf_token).await? {
            Some(pkce) => pkce,
            None => return Err(Error::NoMatched),
        };
        if expired_at < time::OffsetDateTime::now_utc() {
            return Err(Error::Expired);
        }

        let access_token = self.repository.verify_code(&code, &code_verifier).await?;
        let user_info = self.repository.user_info_in_google(&access_token).await?;
        let user = match self
            .repository
            .find_user_by_oauth_sub(&user_info.sub)
            .await?
        {
            Some(user) => user,
            None => {
                self.repository
                    .new_user(user_info.email_address, user_info.name, user_info.image_url)
                    .await?
            }
        };

        let session_id = self
            .repository
            .new_session(user.user_id, user.name, user.image_url)
            .await?;

        self.repository.logged_in_event(user.user_id).await?;

        Ok(Data { session_id })
    }
}

impl CsrfToken {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Code {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CodeVerifier {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
