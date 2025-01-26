use crate::room::domain::RoomCommand;
use postbox::{Message, PostboxError};
use refinement::UserId;

#[derive(Clone)]
pub struct EnterCommand {
    pub(crate) user_id: UserId,
}

#[derive(Clone)]
pub enum EnterError {
    Postbox(PostboxError),
    UserNotFound,
}

impl RoomCommand {
    pub async fn enter(&self, user_id: UserId) -> <EnterCommand as Message>::Executed {
        self.ask(EnterCommand { user_id })
            .await
            .unwrap_or_else(|error| Err(EnterError::Postbox(error)))
    }
}

impl Message for EnterCommand {
    type Executed = Result<(), EnterError>;
}
