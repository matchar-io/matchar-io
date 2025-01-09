pub trait Repository {
    //
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Data {
    pub redirect_url: RedirectUrl,
    pub code_verifier: CodeVerifier,
}

pub struct RedirectUrl(pub String);

pub struct CodeVerifier(pub String);

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
        let redirect_url = "redirect_url".to_string();
        let code_verifier = "code_verifier".to_string();

        Ok(Data {
            redirect_url: RedirectUrl(redirect_url),
            code_verifier: CodeVerifier(code_verifier),
        })
    }
}
