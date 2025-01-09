pub trait Repository {
    fn new_pkce(&self) -> Result<Pkce, Error>;

    async fn pkce_session(
        &self,
        csrf_token: CsrfToken,
        code_verifier: CodeVerifier,
    ) -> Result<(), Error>;
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Pkce {
    pub redirect_url: RedirectUrl,
    pub csrf_token: CsrfToken,
    pub code_verifier: CodeVerifier,
}

pub struct Data {
    pub redirect_url: RedirectUrl,
}

pub struct RedirectUrl(pub String);

pub struct CsrfToken(pub String);

pub struct CodeVerifier(pub String);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create PKCE: {0}")]
    Pkce(anyhow::Error),
}

impl<R> Service<R>
where
    R: Repository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Data, Error> {
        let Pkce {
            redirect_url,
            csrf_token,
            code_verifier,
        } = self.repository.new_pkce()?;
        self.repository
            .pkce_session(csrf_token, code_verifier)
            .await
            .map_err(|error| Error::Pkce(error.into()))?;

        Ok(Data { redirect_url })
    }
}
