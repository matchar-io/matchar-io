use refinement::{EmailAddress, ImageUrl, PkceId, SessionId, UserId, UserName};

pub trait Repository {
    type AccessToken;

    async fn find_pkce_by_csrf_token(
        &self,
        csrf_token: &CsrfToken,
    ) -> Result<Option<PkceEntity>, Error>;

    async fn verify_code(
        &self,
        pkce_id: PkceId,
        code: &Code,
        code_verifier: &CodeVerifier,
    ) -> Result<Self::AccessToken, Error>;

    async fn user_info_in_google(
        &self,
        access_token: &Self::AccessToken,
    ) -> Result<UserInfo, Error>;

    async fn find_user_by_oauth_sub(
        &self,
        sub: &GoogleSubject,
    ) -> Result<Option<UserEntity>, Error>;

    async fn new_user(
        &self,
        sub: &GoogleSubject,
        email_address: &EmailAddress,
        name: &Option<UserName>,
        image_url: &Option<ImageUrl>,
    ) -> Result<UserEntity, Error>;

    async fn new_session(&self, user_id: UserId) -> Result<SessionId, Error>;

    async fn logged_in_event(&self, user_id: UserId) -> Result<(), Error>;
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Data {
    pub session_id: SessionId,
    pub name: UserName,
    pub image_url: ImageUrl,
    pub from_url: url::Url,
}

pub struct CsrfToken(String);

pub struct Code(String);

pub struct CodeVerifier(String);

pub struct PkceEntity {
    pub pkce_id: PkceId,
    pub code_verifier: CodeVerifier,
    pub from_url: url::Url,
    pub expired_at: time::OffsetDateTime,
}

pub struct UserInfo {
    pub sub: GoogleSubject,
    pub email_address: EmailAddress,
    pub name: Option<UserName>,
    pub image_url: Option<ImageUrl>,
}

pub struct GoogleSubject(String);

pub struct UserEntity {
    pub user_id: UserId,
    pub email_address: EmailAddress,
    pub name: UserName,
    pub image_url: ImageUrl,
}

pub struct UserToken(pub String);

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

    pub async fn execute(&self, code: String, csrf_token: String) -> Result<Data, Error> {
        let code = Code(code);
        let csrf_token = CsrfToken(csrf_token);

        let PkceEntity {
            pkce_id,
            code_verifier,
            from_url,
            expired_at,
        } = match self.repository.find_pkce_by_csrf_token(&csrf_token).await? {
            Some(pkce) => pkce,
            None => return Err(Error::NoMatched),
        };
        if expired_at < time::OffsetDateTime::now_utc() {
            return Err(Error::Expired);
        }

        let access_token = self
            .repository
            .verify_code(pkce_id, &code, &code_verifier)
            .await?;
        let user_info = self.repository.user_info_in_google(&access_token).await?;
        let user = match self
            .repository
            .find_user_by_oauth_sub(&user_info.sub)
            .await?
        {
            Some(user) => user,
            None => {
                self.repository
                    .new_user(
                        &user_info.sub,
                        &user_info.email_address,
                        &user_info.name,
                        &user_info.image_url,
                    )
                    .await?
            }
        };

        let session_id = self.repository.new_session(user.user_id).await?;

        self.repository.logged_in_event(user.user_id).await?;

        Ok(Data {
            session_id,
            name: user.name,
            image_url: user.image_url,
            from_url,
        })
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

impl GoogleSubject {
    pub const fn new(sub: String) -> Self {
        Self(sub)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PkceEntity {
    pub fn new(
        pkce_id: PkceId,
        code_verifier: String,
        from_url: url::Url,
        expired_at: time::OffsetDateTime,
    ) -> Self {
        Self {
            pkce_id,
            code_verifier: CodeVerifier(code_verifier.to_owned()),
            from_url,
            expired_at,
        }
    }
}
