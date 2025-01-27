use super::{outbound, Error};
use refinement::SessionId;

pub trait Port: Sync + Send + 'static {
    type User: UserPort;

    fn user(&self) -> &Self::User;
}

pub trait UserPort {
    async fn find_by_session_id(
        &self,
        session_id: SessionId,
    ) -> Result<Option<outbound::UserEntity>, Error>;
}
