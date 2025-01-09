pub trait Repository {
    async fn verify_code(&self, code: String, code_verifier: String) -> Result<AccessToken, Error>;

    async fn user_info(&self, access_token: AccessToken) -> Result<UserInfo, Error>;

    async fn logged_in_event(&self, email_address: EmailAddress) -> Result<(), Error>;
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Data {
    pub token: UserToken,
}

pub struct AccessToken(pub String);

pub struct UserInfo {
    pub sub: String,
    pub email_address: EmailAddress,
    pub name: Option<String>,
    pub image_url: Option<String>,
}

pub struct UserToken(pub String);

#[derive(Debug, Error)]
pub enum Error {
    //
}

impl<R> Service<R>
where
    R: Repository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Data, Error> {
        let token = "token".to_string();

        Ok(Data {
            token: UserToken(token),
        })
    }
}
