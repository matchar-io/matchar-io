use crate::channel::domain::ChannelCommand;
use postbox::{Message, PostboxError};
use refinement::UserId;

pub struct EnterCommand {
    pub(crate) user_id: UserId,
}

pub enum EnterCommandError {
    Postbox(PostboxError),
    UserNotFound,
}

impl ChannelCommand {
    pub async fn enter(&self, user_id: UserId) -> <EnterCommand as Message>::Response {
        match self.ask(EnterCommand { user_id }).await {
            Ok(response) => response,
            Err(error) => Err(EnterCommandError::Postbox(error)),
        }
    }
}

impl Message for EnterCommand {
    type Response = Result<(), EnterCommandError>;
}
