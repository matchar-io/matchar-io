use super::{inbound, outbound, Repository, UserRepository};

pub trait UseCase {
    async fn me_information(&self, data: inbound::Data) -> Result<outbound::Data, Error>;
}

pub struct Service<R> {
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

impl<R> Service<R> {
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> UseCase for Service<R>
where
    R: Repository,
{
    async fn me_information(
        &self,
        inbound::Data { session_id }: inbound::Data,
    ) -> Result<outbound::Data, Error> {
        let now = time::OffsetDateTime::now_utc();
        let user = match self
            .repository
            .user()
            .find_by_session_id(session_id)
            .await?
        {
            Some(user) if user.deactivated_at < now => return Err(Error::Deactivated),
            Some(user) if user.locked_at < now => return Err(Error::Locked),
            Some(user) => user,
            None => return Err(Error::NoMatched),
        };

        Ok(outbound::Data {
            user_id: user.user_id,
            name: user.name,
            image_url: user.image_url,
        })
    }
}
