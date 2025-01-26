use crate::{
    channel::{Channel, ChannelPostbox},
    user::UserPostbox,
};
use postbox::{Context, Handler, Message, PostboxError};
use refinement::UserId;

pub struct Chat {
    user_id: UserId,
    message: String,
}

pub enum ChatError {
    Postbox(PostboxError),
    UserNotFound,
}

impl ChannelPostbox {
    pub async fn chat(&self, user_id: UserId, message: String) -> <Chat as Message>::Response {
        match self.postbox.ask(Chat { user_id, message }).await {
            Ok(response) => response,
            Err(error) => Err(ChatError::Postbox(error)),
        }
    }
}

impl Message for Chat {
    type Response = Result<(), ChatError>;
}

#[postbox::async_trait]
impl Handler<Chat> for Channel {
    type Response = <Chat as Message>::Response;

    async fn handle(
        &mut self,
        Chat { user_id, message }: Chat,
        context: &mut Context<Self>,
    ) -> Self::Response {
        let user: UserPostbox = context
            .registry
            .get(user_id.as_uuid())
            .ok_or(ChatError::UserNotFound)?
            .into();
        user.chat(message).map_err(ChatError::Postbox)?;

        Ok(())
    }
}
