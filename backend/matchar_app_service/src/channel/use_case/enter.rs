use crate::{
    channel::{Channel, ChannelPostbox},
    user::User,
};
use postbox::{Context, Handler, Message, PostboxError};
use refinement::UserId;

pub struct Enter {
    user_id: UserId,
}

pub enum EnterError {
    Postbox(PostboxError),
    UserNotFound,
}

impl ChannelPostbox {
    pub async fn enter(&self, user_id: UserId) -> Result<(), EnterError> {
        match self.postbox.ask(Enter { user_id }).await {
            Ok(response) => response,
            Err(error) => Err(EnterError::Postbox(error)),
        }
    }
}

impl Message for Enter {
    type Response = Result<(), EnterError>;
}

#[postbox::async_trait]
impl Handler<Enter> for Channel {
    type Response = Result<(), EnterError>;

    async fn handle(&mut self, message: Enter, context: &mut Context<Self>) -> Self::Response {
        match context.registry.get::<User>(message.user_id.as_uuid()) {
            Some(user) => {
                self.users.insert(message.user_id, user.into());

                Ok(())
            }
            None => Err(EnterError::UserNotFound),
        }
    }
}
