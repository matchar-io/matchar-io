use crate::channel::domain::ChannelCommand;
use postbox::{Message, PostboxError};
use refinement::{UserId, UserName};

pub struct ChatEvent {
    pub(crate) user: User,
    pub(crate) message: String,
}

pub struct User {
    pub(crate) user_id: UserId,
    pub(crate) name: UserName,
}

pub enum ChatEventError {
    Postbox(PostboxError),
}

impl ChannelCommand {
    pub async fn chat(&self, user: User, message: String) -> <ChatEvent as Message>::Response {
        match self.ask(ChatEvent { user, message }).await {
            Ok(response) => response,
            Err(error) => Err(ChatEventError::Postbox(error)),
        }
    }
}

impl Message for ChatEvent {
    type Response = Result<(), ChatEventError>;
}
