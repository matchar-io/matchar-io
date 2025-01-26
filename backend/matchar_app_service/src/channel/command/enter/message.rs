use crate::channel::domain::ChannelCommand;
use postbox::{Message, PostboxError};
use refinement::UserId;

#[derive(Clone)]
pub struct EnterCommand {
    pub(crate) user_id: UserId,
}

#[derive(Clone)]
pub enum EnterCommandError {
    Postbox(PostboxError),
    UserNotFound,
}

impl ChannelCommand {
    pub async fn enter(&self, user_id: UserId) -> <EnterCommand as Message>::Executed {
        self.ask(EnterCommand { user_id })
            .await
            .unwrap_or_else(|error| Err(EnterCommandError::Postbox(error)))
    }
}

impl Message for EnterCommand {
    type Executed = Result<(), EnterCommandError>;
}
