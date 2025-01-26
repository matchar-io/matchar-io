use crate::room::domain::RoomCommand;
use postbox::{Message, PostboxError};
use refinement::UserId;

pub struct EnterCommand {
    pub(crate) user_id: UserId,
}

pub enum EnterError {
    Postbox(PostboxError),
    UserNotFound,
}

impl RoomCommand {
    pub async fn enter(&self, user_id: UserId) -> <EnterCommand as Message>::Response {
        match self.ask(EnterCommand { user_id }).await {
            Ok(response) => response,
            Err(error) => Err(EnterError::Postbox(error)),
        }
    }
}

impl Message for EnterCommand {
    type Response = Result<(), EnterError>;
}
