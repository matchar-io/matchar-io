use super::{outbound, Error};
use refinement::SessionId;

pub trait Repository: Sync + Send + 'static {
    type User: UserRepository;

    fn user(&self) -> &Self::User;
}

pub trait UserRepository {
    async fn find_by_session_id(
        &self,
        session_id: SessionId,
    ) -> Result<Option<outbound::UserEntity>, Error>;
}
