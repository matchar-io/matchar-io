use crate::{
    channel::{Channel, ChannelPostbox},
    user::UserPostbox,
};
use postbox::{Context, Handler, Message, PostboxError};
use refinement::UserId;

pub struct ChatInChannel {
    user_id: UserId,
    message: String,
}

pub enum ChatInChannelError {
    Postbox(PostboxError),
    UserNotFound,
}

impl ChannelPostbox {
    pub async fn chat(
        &self,
        user_id: UserId,
        message: String,
    ) -> <ChatInChannel as Message>::Response {
        match self.postbox.ask(ChatInChannel { user_id, message }).await {
            Ok(response) => response,
            Err(error) => Err(ChatInChannelError::Postbox(error)),
        }
    }
}

impl Message for ChatInChannel {
    type Response = Result<(), ChatInChannelError>;
}

#[postbox::async_trait]
impl Handler<ChatInChannel> for Channel {
    type Response = <ChatInChannel as Message>::Response;

    async fn handle(
        &mut self,
        ChatInChannel { user_id, message }: ChatInChannel,
        context: &mut Context<Self>,
    ) -> Self::Response {
        let user: UserPostbox = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(ChatInChannelError::UserNotFound)?
            .into();
        user.chat(message).map_err(ChatInChannelError::Postbox)?;

        Ok(())
    }
}
