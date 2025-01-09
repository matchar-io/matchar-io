use refinement::EmailAddress;

// TODO: refinement 내에 Id 타입을 정의하고 사용하도록 수정.
pub type UserId = String;

pub type SessionId = String;

pub trait Repository {
    async fn verify_code(
        &self,
        code: String,
        code_verifier: String,
        csrf_token: String,
    ) -> Result<AccessToken, Error>;

    async fn user_info_in_google(&self, access_token: AccessToken) -> Result<UserInfo, Error>;

    async fn find_user(&self, sub: String) -> Result<Option<UserEntity>, Error>;

    async fn new_user(
        &self,
        email_address: EmailAddress,
        name: String,
        image_url: Option<String>,
    ) -> Result<UserEntity, Error>;

    async fn new_session(
        &self,
        user_id: UserId,
        name: String,
        image_url: String,
    ) -> Result<SessionId, Error>;

    fn session_token(&self, session_id: SessionId) -> SessionToken;

    async fn logged_in_event(&self, user_id: UserId) -> Result<(), Error>;
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Data {
    pub session_token: SessionToken,
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
    #[error("Invalid CSRF token")]
    InvalidCsrfToken,
}

impl<R> Service<R>
where
    R: Repository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        code: String,
        code_verifier: String,
        csrf_token: String,
    ) -> Result<Data, Error> {
        let access_token = self
            .repository
            .verify_code(code, code_verifier, csrf_token)
            .await?;
        let user_info = self.repository.user_info_in_google(access_token).await?;
        let user = match self.repository.find_user(user_info.sub).await? {
            Some(user) => user,
            None => {
                self.repository
                    .new_user(
                        user_info.email_address.clone(),
                        user_info.name.clone().unwrap_or_default(),
                        user_info.image_url.clone(),
                    )
                    .await?
            }
        };

        let session_id = self
            .repository
            .new_session(user.user_id.clone(), user.name, user.image_url)
            .await?;
        let session_token = self.repository.session_token(session_id);

        self.repository
            .logged_in_event(user.user_id.clone())
            .await?;

        Ok(Data { session_token })
    }
}
