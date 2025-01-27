use super::{outbound, Error};
use refinement::UserId;

pub trait Port: Sync + Send + 'static {
    type User: UserPort;

    fn user(&self) -> &Self::User;
}

pub trait UserPort {
    async fn find_by_user_id(&self, user_id: UserId)
        -> Result<Option<outbound::UserEntity>, Error>;
}
