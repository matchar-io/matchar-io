use refinement::{ImageUrl, SessionId, UserId, UserName};

pub trait Repository {
    async fn find_user_by_session_id(
        &self,
        session_id: SessionId,
    ) -> Result<Option<UserEntity>, Error>;
}

pub struct Service<R: Repository> {
    repository: R,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("No matched")]
    NoMatched,
    #[error("User is deactivated")]
    Deactivated,
    #[error("User is locked")]
    Locked,

    #[error("Database error: {0}")]
    DatabaseError(anyhow::Error),
}

pub struct Data {
    pub user_id: UserId,
    pub name: UserName,
    pub image_url: ImageUrl,
}

pub struct UserEntity {
    pub user_id: UserId,
    pub name: UserName,
    pub image_url: ImageUrl,
    pub deactivated_at: time::OffsetDateTime,
    pub locked_at: time::OffsetDateTime,
}

impl<R> Service<R>
where
    R: Repository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, session_id: SessionId) -> Result<Data, Error> {
        let now = time::OffsetDateTime::now_utc();
        let user = match self.repository.find_user_by_session_id(session_id).await? {
            Some(user) if user.deactivated_at < now => return Err(Error::Deactivated),
            Some(user) if user.locked_at < now => return Err(Error::Locked),
            Some(user) => user,
            None => return Err(Error::NoMatched),
        };

        Ok(Data {
            user_id: user.user_id,
            name: user.name,
            image_url: user.image_url,
        })
    }
}
