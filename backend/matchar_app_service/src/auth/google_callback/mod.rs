pub trait Repository {
    //
}

pub struct Service<R: Repository> {
    repository: R,
}

pub struct Data {
    pub token: UserToken,
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
